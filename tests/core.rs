use serde_json::{Value, json};
use upgrade_witness::{
    compare::compare_captures,
    config::parse_config,
    report::{escape, redact, relative_artifact, sha256},
    types::*,
};

fn config() -> WitnessConfig {
    parse_config(&json!({"schemaVersion":1,"scenarios":[{"id":"main","route":"/","screenshot":false,"assertions":[{"id":"value","selector":"h1","kind":"text","expected":"before"}]}]}).to_string()).unwrap()
}
fn capture(value: Value) -> ScenarioCapture {
    ScenarioCapture {
        scenario_id: "main".into(),
        checks: vec![Check {
            id: "value".into(),
            expected: json!("irrelevant"),
            actual: value,
            passed: false,
            error: None,
        }],
        screenshot_path: None,
        error: None,
        browser_version: "fixture".into(),
    }
}
fn result(c: &WitnessConfig, a: ScenarioCapture, b: ScenarioCapture) -> Comparison {
    compare_captures(
        c,
        std::slice::from_ref(&a),
        std::slice::from_ref(&b),
        Some(std::slice::from_ref(&a)),
        Some(std::slice::from_ref(&b)),
    )
}

#[test]
fn config_defaults_and_unique_ids() {
    let c = config();
    assert_eq!(c.timeout_ms, 15000);
    assert_eq!(c.scenarios[0].viewport.width, 1440);
    let mut v = serde_json::to_value(c).unwrap();
    let first = v["scenarios"][0].clone();
    v["scenarios"].as_array_mut().unwrap().push(first);
    assert!(parse_config(&v.to_string()).is_err());
}
#[test]
fn invalid_config_does_not_silently_drop_fields() {
    let mut v = serde_json::to_value(config()).unwrap();
    v["extra"] = json!(1);
    assert!(parse_config(&v.to_string()).is_err());
}
#[test]
fn routes_and_expected_types_are_validated() {
    let mut v = serde_json::to_value(config()).unwrap();
    v["scenarios"][0]["route"] = json!("//elsewhere.example");
    assert!(parse_config(&v.to_string()).is_err());
    v["scenarios"][0]["route"] = json!("/");
    v["scenarios"][0]["assertions"][0]["expected"] = json!(true);
    assert!(parse_config(&v.to_string()).is_err());
}
#[test]
fn unknown_intent_is_rejected() {
    let mut v = serde_json::to_value(config()).unwrap();
    v["intendedChanges"] = json!([{"scenarioId":"main","assertionId":"missing","expected":"after","reason":"Reviewed new behavior"}]);
    assert!(parse_config(&v.to_string()).is_err());
}
#[test]
fn clean_result_recomputes_actual_not_passed_flags() {
    let r = result(
        &config(),
        capture(json!("before")),
        capture(json!("before")),
    );
    assert_eq!(r.exit_code, 0);
    assert!(r.complete);
    assert_eq!(r.coverage.passed, 2);
}
#[test]
fn regression_is_repeatable_and_has_coverage() {
    let r = result(&config(), capture(json!("before")), capture(json!("after")));
    assert_eq!(r.exit_code, 1);
    assert!(r.complete);
    assert_eq!(r.coverage.failed, 1);
    assert_eq!(r.findings[0].kind, "regression");
}
#[test]
fn intended_change_is_scoped_to_matching_expectation() {
    let mut c = config();
    c.intended_changes.push(Intent {
        scenario_id: "main".into(),
        assertion_id: "value".into(),
        expected: json!("after"),
        reason: "Approved new text".into(),
    });
    let r = result(&c, capture(json!("before")), capture(json!("after")));
    assert_eq!(r.exit_code, 0);
    assert_eq!(r.findings[0].kind, "intended-change");
    assert_eq!(
        result(&c, capture(json!("before")), capture(json!("wrong"))).exit_code,
        1
    );
}
#[test]
fn failing_baseline_is_not_a_new_regression() {
    let r = result(&config(), capture(json!("wrong")), capture(json!("after")));
    assert_eq!(r.exit_code, 2);
    assert!(r.findings.iter().all(|f| f.kind != "regression"));
}
#[test]
fn missing_observations_are_inconclusive() {
    let r = compare_captures(&config(), &[], &[], None, None);
    assert_eq!(r.exit_code, 2);
    assert_eq!(r.coverage.not_run, 2);
    assert!(!r.complete);
}
#[test]
fn duplicate_observations_are_inconclusive() {
    let c = capture(json!("before"));
    let r = compare_captures(&config(), &[c.clone(), c.clone()], &[c], None, None);
    assert_eq!(r.exit_code, 2);
}
#[test]
fn unstable_baseline_prevents_regression() {
    let r = compare_captures(
        &config(),
        &[capture(json!("before"))],
        &[capture(json!("after"))],
        Some(&[capture(json!("different"))]),
        None,
    );
    assert_eq!(r.exit_code, 2);
    assert!(r.findings.iter().all(|f| f.kind != "regression"));
}
#[test]
fn unstable_head_prevents_regression() {
    let r = compare_captures(
        &config(),
        &[capture(json!("before"))],
        &[capture(json!("after"))],
        None,
        Some(&[capture(json!("before"))]),
    );
    assert_eq!(r.exit_code, 2);
}
#[test]
fn hidden_false_is_valid_observation() {
    let mut c = config();
    c.scenarios[0].assertions[0].kind = "visible".into();
    c.scenarios[0].assertions[0].expected = json!(false);
    let r = result(&c, capture(json!(false)), capture(json!(false)));
    assert_eq!(r.exit_code, 0);
}
#[test]
fn pixels_alone_require_review() {
    let temp = tempfile::tempdir().unwrap();
    let a = temp.path().join("a.png");
    let b = temp.path().join("b.png");
    image::RgbaImage::from_pixel(10, 10, image::Rgba([0, 0, 0, 255]))
        .save(&a)
        .unwrap();
    image::RgbaImage::from_pixel(10, 10, image::Rgba([255, 255, 255, 255]))
        .save(&b)
        .unwrap();
    let mut c = config();
    c.scenarios[0].screenshot = true;
    let mut before = capture(json!("before"));
    before.screenshot_path = Some(a.to_string_lossy().into());
    let mut after = capture(json!("before"));
    after.screenshot_path = Some(b.to_string_lossy().into());
    let r = result(&c, before, after);
    assert_eq!(r.exit_code, 2);
    assert_eq!(r.findings[0].kind, "needs-review");
}
#[test]
fn missing_image_keeps_confirmed_assertion_but_marks_incomplete() {
    let mut c = config();
    c.scenarios[0].screenshot = true;
    let r = result(&c, capture(json!("before")), capture(json!("after")));
    assert_eq!(r.exit_code, 1);
    assert!(!r.complete);
}
#[test]
fn redaction_and_html_escaping_preserve_unicode() {
    assert_eq!(redact("Bearer abc.def"), "Bearer [REDACTED]");
    assert_eq!(
        redact("https://x/?token=secret&ok=1"),
        "https://x/?token=[REDACTED]&ok=1"
    );
    assert!(!redact("ghp_12345678901234567890123456789").contains("123456"));
    assert_eq!(escape("Nâng <x>"), "Nâng &lt;x&gt;");
}
#[test]
fn artifact_boundaries_require_existing_owned_files() {
    let t = tempfile::tempdir().unwrap();
    let root = t.path().join("run");
    std::fs::create_dir(&root).unwrap();
    let outside = t.path().join("outside");
    std::fs::write(&outside, "secret").unwrap();
    assert!(relative_artifact(&root, &outside).is_err());
    let inside = root.join("ok");
    std::fs::write(&inside, "ok").unwrap();
    assert_eq!(relative_artifact(&root, &inside).unwrap(), "ok");
}
#[test]
fn hashes_identify_exact_bytes() {
    assert_eq!(
        sha256(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn exported_report_escapes_page_text_and_redacts_known_tokens() {
    let out = tempfile::tempdir().unwrap();
    let report = RunReport {
        schema_version: "0.2".into(),
        tool_version: "test".into(),
        run_id: "test".into(),
        created_at: "2026-10-17T00:00:00Z".into(),
        base_sha: "base".into(),
        head_sha: "head".into(),
        contract_sha256: sha256(b"contract"),
        config: config(),
        environment: json!({"runtime":"Rust"}),
        complete: false,
        exit_code: 2,
        findings: vec![Finding {
            id: "one".into(),
            scenario_id: "main".into(),
            assertion_id: None,
            kind: "needs-review".into(),
            observation: "<script>alert(1)</script> ghp_12345678901234567890123456789".into(),
            base: None,
            head: None,
            expected: None,
            reason: None,
            evidence_ids: vec![],
        }],
        evidence: vec![],
        coverage: Coverage {
            planned: 2,
            not_run: 2,
            ..Coverage::default()
        },
        limitations: vec!["Bounded coverage".into()],
    };
    upgrade_witness::report::write_report(&report, out.path()).unwrap();
    let html = std::fs::read_to_string(out.path().join("report.html")).unwrap();
    assert!(html.contains("&lt;script&gt;"));
    assert!(!html.contains("<script>"));
    assert!(!html.contains("1234567890"));
    let json = std::fs::read_to_string(out.path().join("report.json")).unwrap();
    assert!(json.contains("[REDACTED]"));
    assert_eq!(serde_json::from_str::<Value>(&json).unwrap()["exitCode"], 2);
}

#[test]
fn report_rejects_external_artifact_links_before_export() {
    let out = tempfile::tempdir().unwrap();
    let mut report:RunReport=serde_json::from_value(json!({
        "schemaVersion":"0.2","toolVersion":"test","runId":"test","createdAt":"test","baseSha":"b","headSha":"h","contractSha256":"d",
        "config":config(),"environment":{},"complete":false,"exitCode":2,"findings":[],"coverage":{"planned":0,"passed":0,"failed":0,"notRun":0},"limitations":[],
        "evidence":[{"id":"bad","path":"javascript:alert(1)","sha256":"fake","mediaType":"image/png"}]
    })).unwrap();
    assert!(upgrade_witness::report::write_report(&report, out.path()).is_err());
    report.evidence[0].path = "../private.png".into();
    assert!(upgrade_witness::report::write_report(&report, out.path()).is_err());
    assert!(!out.path().join("report.html").exists());
}
