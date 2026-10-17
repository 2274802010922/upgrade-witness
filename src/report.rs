use crate::types::*;
use anyhow::{Result, bail};
use regex::Regex;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Component, Path},
    sync::LazyLock,
};

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn redact(text: &str) -> String {
    static BEARER: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)\bBearer\s+[A-Za-z0-9._~+/-]+=*").expect("static regex"));
    static TOKEN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"\b(gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,}|sk-[A-Za-z0-9_-]{20,})\b",
        )
        .expect("static regex")
    });
    static QUERY: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)([?&](?:token|access_token|api_key|secret|password)=)[^&#\s]+")
            .expect("static regex")
    });
    let value = BEARER.replace_all(text, "Bearer [REDACTED]");
    let value = TOKEN.replace_all(&value, "[REDACTED]");
    QUERY.replace_all(&value, "${1}[REDACTED]").into_owned()
}
pub fn sanitize(value: &mut Value) {
    match value {
        Value::String(s) => *s = redact(s),
        Value::Array(v) => v.iter_mut().for_each(sanitize),
        Value::Object(v) => v.values_mut().for_each(sanitize),
        _ => (),
    }
}
pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
fn href(text: &str) -> String {
    if text.is_empty()
        || text.starts_with('/')
        || text.split('/').any(|s| s == "..")
        || !text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "._-/".contains(c))
    {
        "#invalid-artifact".into()
    } else {
        text.into()
    }
}
fn cell(text: &str) -> String {
    text.replace('|', "\\|").replace(['\n', '\r'], " ")
}
fn display(v: &Option<Value>) -> String {
    match v {
        Some(Value::String(s)) => s.clone(),
        Some(v) => v.to_string(),
        None => "—".into(),
    }
}

pub fn relative_artifact(root: &Path, file: &Path) -> Result<String> {
    let root = fs::canonicalize(root)?;
    let file = fs::canonicalize(file)?;
    let relative = file
        .strip_prefix(&root)
        .map_err(|_| anyhow::anyhow!("Evidence path must stay within the report directory"))?;
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        bail!("Invalid evidence path");
    }
    Ok(relative.to_string_lossy().replace('\\', "/"))
}
pub fn collect_evidence(
    root: &Path,
    groups: &[(&str, &[ScenarioCapture])],
) -> Result<Vec<Evidence>> {
    let mut evidence = vec![];
    for (label, captures) in groups {
        let mut normalized = captures.to_vec();
        for capture in &mut normalized {
            if let Some(file) = &capture.screenshot_path {
                let relative = relative_artifact(root, Path::new(file))?;
                evidence.push(Evidence {
                    id: format!("{label}-{}", capture.scenario_id),
                    path: relative.clone(),
                    sha256: sha256(&fs::read(file)?),
                    media_type: "image/png".into(),
                });
                capture.screenshot_path = Some(relative);
            }
        }
        fs::create_dir_all(root.join(label))?;
        let mut json = serde_json::to_value(normalized)?;
        sanitize(&mut json);
        let bytes = serde_json::to_vec_pretty(&json)?;
        let file = format!("{label}/checks.json");
        fs::write(root.join(&file), &bytes)?;
        evidence.push(Evidence {
            id: format!("{label}-checks"),
            path: file,
            sha256: sha256(&bytes),
            media_type: "application/json".into(),
        });
    }
    Ok(evidence)
}

pub fn render_markdown(r: &RunReport) -> String {
    let mut text = format!(
        "# Upgrade Witness report\n\nRust CLI {} · Run `{}` · Exit **{}** · Complete **{}**\n\nBase: `{}`\n\nHead: `{}`\n\nCoverage: {} passed, {} failed, {} not run / {} planned checkpoints.\n\n| Kind | Scenario | Assertion | Observation |\n|---|---|---|---|\n",
        r.tool_version,
        r.run_id,
        r.exit_code,
        r.complete,
        r.base_sha,
        r.head_sha,
        r.coverage.passed,
        r.coverage.failed,
        r.coverage.not_run,
        r.coverage.planned
    );
    for f in &r.findings {
        text += &format!(
            "| {} | {} | {} | {} |\n",
            cell(&f.kind),
            cell(&f.scenario_id),
            cell(f.assertion_id.as_deref().unwrap_or("—")),
            cell(&f.observation)
        );
    }
    text += "\n## Evidence\n\n";
    for e in &r.evidence {
        text += &format!(
            "- [{}]({}) — SHA-256 `{}`\n",
            cell(&e.id),
            href(&e.path),
            e.sha256
        );
    }
    text += "\n## Limits\n\n";
    for limit in &r.limitations {
        text += &format!("- {limit}\n");
    }
    text
}

pub fn render_html(r: &RunReport) -> String {
    let mut cards = String::new();
    for f in &r.findings {
        cards += &format!(
            "<article class=\"finding {}\"><span class=\"kind\">{}</span><h3>{} / {}</h3><p>{}</p>",
            escape(&f.kind),
            escape(&f.kind.to_uppercase()),
            escape(&f.scenario_id),
            escape(f.assertion_id.as_deref().unwrap_or("visual")),
            escape(&f.observation)
        );
        if f.base.is_some() {
            cards += &format!(
                "<pre>Base: {}\nHead: {}\nExpected: {}</pre>",
                escape(&display(&f.base)),
                escape(&display(&f.head)),
                escape(&display(&f.expected))
            );
        }
        if let Some(reason) = &f.reason {
            cards += &format!("<p>{}</p>", escape(reason));
        }
        cards += &format!(
            "<p class=\"proof\">Evidence: {}</p></article>",
            escape(&f.evidence_ids.join(", "))
        );
    }
    if cards.is_empty() {
        cards="<article class=\"finding\"><h3>No regression observed</h3><p>This applies only to completed declared checks.</p></article>".into();
    }
    let mut images: Vec<_> = r
        .evidence
        .iter()
        .filter(|e| {
            e.media_type == "image/png"
                && !e.id.starts_with("base-repeat-")
                && !e.id.starts_with("head-repeat-")
        })
        .collect();
    images.sort_by_key(|e| {
        (
            e.id.strip_prefix("base-")
                .or_else(|| e.id.strip_prefix("head-"))
                .unwrap_or(&e.id)
                .to_string(),
            e.id.clone(),
        )
    });
    let pictures=images.iter().map(|e|format!("<figure><figcaption>{}</figcaption><a href=\"{}\"><img src=\"{}\" alt=\"Captured {}\" loading=\"lazy\"></a><p class=\"proof\"><code>{}</code></p></figure>",escape(&e.id),escape(&href(&e.path)),escape(&href(&e.path)),escape(&e.id),escape(&e.sha256))).collect::<String>();
    let limits = r
        .limitations
        .iter()
        .map(|l| format!("<li>{}</li>", escape(l)))
        .collect::<String>();
    format!(
        r#"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src 'self' data:; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'"><title>Upgrade Witness — Rust report</title><style>{}</style></head><body><main><header><div class="eyebrow">UPGRADE WITNESS / RUST ALPHA</div><h1>See what changed.</h1><p class="muted">Evidence for the scenarios in this run. This is not a whole-application correctness guarantee.</p><span class="status">Exit {}</span><span class="status">{}</span><span class="status">{}/{} checkpoints passed</span><p><strong>Base</strong> <code>{}</code><br><strong>Head</strong> <code>{}</code></p></header><h2>Findings</h2><div class="grid">{}</div><h2>Before and after</h2><div class="grid">{}</div><h2>Coverage and limits</h2><p>{} passed · {} failed · {} not run · {} planned checkpoints.</p><ul>{}</ul><footer><a href="report.json">JSON report</a> · <a href="report.md">Markdown report</a><p class="muted"><code>{}</code> · {}</p></footer></main></body></html>"#,
        include_str!("report.css"),
        r.exit_code,
        if r.complete { "Complete" } else { "Incomplete" },
        r.coverage.passed,
        r.coverage.planned,
        escape(&r.base_sha),
        escape(&r.head_sha),
        cards,
        pictures,
        r.coverage.passed,
        r.coverage.failed,
        r.coverage.not_run,
        r.coverage.planned,
        limits,
        escape(&r.run_id),
        escape(&r.created_at)
    )
}
pub fn write_report(report: &RunReport, out: &Path) -> Result<()> {
    let mut value = serde_json::to_value(report)?;
    sanitize(&mut value);
    let clean: RunReport = serde_json::from_value(value)?;
    for e in &clean.evidence {
        if href(&e.path) == "#invalid-artifact" {
            bail!("Unsafe report artifact link");
        }
        relative_artifact(out, &out.join(&e.path))?;
    }
    fs::write(out.join("report.json"), serde_json::to_vec_pretty(&clean)?)?;
    fs::write(out.join("report.md"), render_markdown(&clean))?;
    fs::write(out.join("report.html"), render_html(&clean))?;
    Ok(())
}
