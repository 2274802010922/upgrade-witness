use crate::{
    capture::capture_scenarios,
    compare::compare_captures,
    config::load_config,
    report::{collect_evidence, redact, sanitize, sha256, write_report},
    types::*,
    workspace::{StartOptions, create_revision_workspaces, prepare_and_start},
};
use anyhow::{Result, bail};
use serde_json::json;
use std::{fs, path::Path};

pub fn verify(
    repo: &Path,
    base_ref: &str,
    head_ref: &str,
    contract: &Path,
    out: &Path,
) -> Result<RunReport> {
    let config = load_config(contract)?;
    if out.exists() {
        bail!("Output directory must be new; existing runs are never overwritten");
    }
    fs::create_dir_all(out)?;
    let contract_bytes = serde_json::to_vec_pretty(&config)?;
    let digest = sha256(&contract_bytes);
    let mut exported = serde_json::to_value(&config)?;
    sanitize(&mut exported);
    let export_bytes = serde_json::to_vec_pretty(&exported)?;
    fs::write(out.join("contract.json"), &export_bytes)?;
    let planned = config
        .scenarios
        .iter()
        .map(|s| s.assertions.len() * 2)
        .sum();
    let mut report=RunReport {schema_version:"0.2".into(),tool_version:env!("CARGO_PKG_VERSION").into(),run_id:uuid::Uuid::new_v4().to_string(),created_at:chrono::Utc::now().to_rfc3339(),base_sha:String::new(),head_sha:String::new(),contract_sha256:digest.clone(),config:config.clone(),environment:json!({"runtime":"native Rust","platform":std::env::consts::OS,"arch":std::env::consts::ARCH,"browser":"not started","driver":"headless_chrome 1.0.22"}),complete:false,exit_code:2,findings:vec![],evidence:vec![],coverage:Coverage{planned,not_run:planned,..Coverage::default()},limitations:vec![
        "Only declared scenarios and assertion checkpoints are covered; no whole-application correctness guarantee.".into(),
        "Screenshot differences are review items. Rust compares dimensions and marks a pixel changed if any RGBA channel differs by more than 25/255; this small threshold is not a claim of general visual-diff accuracy.".into(),
        "Use synthetic data. Screenshot masks and token redaction are best effort; explicitly collected assertion values and arbitrary page content may contain private data.".into(),
        "The CLI is native Rust with no model API or uploads. A host agent may send selected context to its model provider.".into(),
        "Clock, random values and live network responses are not automatically frozen. Chromium must support navigation responseStatus; unknown HTTP status is inconclusive.".into(),
        "Git worktrees are not security sandboxes. npm is used only to install/build/serve the target frontend. Deliberately daemonized children may escape process cleanup.".into(),
        "contractSha256 binds normalized original input. Exported contract data may be redacted; replay needs the original configuration.".into(),
    ]};
    let (mut base, mut repeated, mut head, mut head_repeated) = (vec![], vec![], vec![], vec![]);
    let operation = (|| -> Result<()> {
        let mut workspace = create_revision_workspaces(repo, base_ref, head_ref, out)?;
        report.base_sha = workspace.base_sha.clone();
        report.head_sha = workspace.head_sha.clone();
        if !workspace.dirty_files.is_empty() {
            report.limitations.push(format!(
                "{} uncommitted source entries were excluded and left untouched.",
                workspace.dirty_files.len()
            ));
        }
        let inputs = json!({"baseSha":workspace.base_sha,"headSha":workspace.head_sha,"contractSha256":digest,"lockfileSha256":{"base":sha256(&fs::read(workspace.base_dir.join("package-lock.json"))?),"head":sha256(&fs::read(workspace.head_dir.join("package-lock.json"))?)},"driver":"headless_chrome 1.0.22","platform":std::env::consts::OS,"arch":std::env::consts::ARCH});
        fs::write(out.join("inputs.json"), serde_json::to_vec_pretty(&inputs)?)?;
        eprintln!("Capturing baseline twice with the Rust browser driver...");
        let mut server = prepare_and_start(&workspace.base_dir, &config, StartOptions::default())?;
        let port = url::Url::parse(&server.url)?.port();
        base = capture_scenarios(&server.url, &config, &out.join("base"), "base");
        repeated = capture_scenarios(
            &server.url,
            &config,
            &out.join("base-repeat"),
            "base-repeat",
        );
        server.stop()?;
        eprintln!("Capturing candidate twice...");
        let mut server = prepare_and_start(
            &workspace.head_dir,
            &config,
            StartOptions {
                install: true,
                port,
            },
        )?;
        head = capture_scenarios(&server.url, &config, &out.join("head"), "head");
        head_repeated = capture_scenarios(
            &server.url,
            &config,
            &out.join("head-repeat"),
            "head-repeat",
        );
        server.stop()?;
        workspace.cleanup()?;
        Ok(())
    })();
    let comparison = compare_captures(&config, &base, &head, Some(&repeated), Some(&head_repeated));
    report.complete = comparison.complete;
    report.exit_code = comparison.exit_code;
    report.findings = comparison.findings;
    report.coverage = comparison.coverage;
    if let Err(error) = operation {
        report.findings.push(Finding {
            id: "infrastructure".into(),
            scenario_id: "run".into(),
            assertion_id: None,
            kind: "inconclusive".into(),
            observation: redact(&error.to_string()),
            base: None,
            head: None,
            expected: None,
            reason: None,
            evidence_ids: vec![],
        });
        report.complete = false;
        if report.exit_code == 0 {
            report.exit_code = 2;
        }
    }
    let browser = base
        .first()
        .or_else(|| head.first())
        .map(|c| c.browser_version.as_str())
        .unwrap_or("not started");
    report.environment["browser"] = json!(browser);
    report.evidence = collect_evidence(
        out,
        &[
            ("base", &base),
            ("base-repeat", &repeated),
            ("head", &head),
            ("head-repeat", &head_repeated),
        ],
    )?;
    report.evidence.push(Evidence {
        id: "contract".into(),
        path: "contract.json".into(),
        sha256: sha256(&export_bytes),
        media_type: "application/json".into(),
    });
    if out.join("inputs.json").is_file() {
        report.evidence.push(Evidence {
            id: "inputs".into(),
            path: "inputs.json".into(),
            sha256: sha256(&fs::read(out.join("inputs.json"))?),
            media_type: "application/json".into(),
        });
    }
    report.limitations.push(format!("Replay using the original repo/config: uw verify --repo <repository> --base {} --head {} --config <original-config> --out <new-directory> --trust-project",report.base_sha,report.head_sha));
    write_report(&report, out)?;
    Ok(report)
}
