use crate::types::*;
use serde_json::Value;
use std::fs;

#[derive(PartialEq)]
enum Visual {
    Same,
    Different,
    Unavailable,
}
fn images(a: Option<&str>, b: Option<&str>, tolerance: f64) -> Visual {
    let result = (|| -> anyhow::Result<Visual> {
        let (Some(a), Some(b)) = (a, b) else {
            return Ok(Visual::Unavailable);
        };
        if fs::metadata(a)?.len() > 32 * 1024 * 1024 || fs::metadata(b)?.len() > 32 * 1024 * 1024 {
            return Ok(Visual::Unavailable);
        }
        let a = image::open(a)?.to_rgba8();
        let b = image::open(b)?.to_rgba8();
        if a.dimensions() != b.dimensions() {
            return Ok(Visual::Different);
        }
        let pixels = u64::from(a.width()) * u64::from(a.height());
        if pixels == 0 {
            return Ok(Visual::Unavailable);
        }
        let different = a
            .pixels()
            .zip(b.pixels())
            .filter(|(x, y)| x.0.iter().zip(y.0.iter()).any(|(c, d)| c.abs_diff(*d) > 25))
            .count();
        Ok(if different as f64 / pixels as f64 > tolerance {
            Visual::Different
        } else {
            Visual::Same
        })
    })();
    result.unwrap_or(Visual::Unavailable)
}
fn one_capture<'a>(captures: &'a [ScenarioCapture], id: &str) -> Option<&'a ScenarioCapture> {
    let mut values = captures.iter().filter(|c| c.scenario_id == id);
    let item = values.next()?;
    if values.next().is_some() {
        None
    } else {
        Some(item)
    }
}
fn one_check<'a>(capture: Option<&'a ScenarioCapture>, id: &str) -> Option<&'a Check> {
    let mut values = capture?.checks.iter().filter(|c| c.id == id);
    let item = values.next()?;
    if values.next().is_some() {
        None
    } else {
        Some(item)
    }
}
fn screenshot(capture: Option<&ScenarioCapture>) -> Option<&str> {
    capture.and_then(|c| c.screenshot_path.as_deref())
}
fn add(findings: &mut Vec<Finding>, template: &Finding, kind: &str, observation: &str) {
    let mut finding = template.clone();
    finding.id = format!("finding-{}", findings.len() + 1);
    finding.kind = kind.into();
    finding.observation = observation.into();
    findings.push(finding);
}

pub fn compare_captures(
    config: &WitnessConfig,
    base: &[ScenarioCapture],
    head: &[ScenarioCapture],
    base_repeat: Option<&[ScenarioCapture]>,
    head_repeat: Option<&[ScenarioCapture]>,
) -> Comparison {
    let mut findings = vec![];
    let mut coverage = Coverage {
        planned: config
            .scenarios
            .iter()
            .map(|s| s.assertions.len() * 2)
            .sum(),
        ..Coverage::default()
    };
    for scenario in &config.scenarios {
        let a = one_capture(base, &scenario.id);
        let b = one_capture(head, &scenario.id);
        let ar = base_repeat.and_then(|v| one_capture(v, &scenario.id));
        let br = head_repeat.and_then(|v| one_capture(v, &scenario.id));
        let evidence_ids = [
            ("base", a),
            ("head", b),
            ("base-repeat", ar),
            ("head-repeat", br),
        ]
        .iter()
        .filter(|(_, v)| v.is_some_and(|c| c.screenshot_path.is_some()))
        .map(|(label, _)| format!("{label}-{}", scenario.id))
        .collect();
        let template = Finding {
            id: String::new(),
            scenario_id: scenario.id.clone(),
            assertion_id: None,
            kind: String::new(),
            observation: String::new(),
            base: None,
            head: None,
            expected: None,
            reason: None,
            evidence_ids,
        };
        if a.is_none_or(|c| c.error.is_some()) || b.is_none_or(|c| c.error.is_some()) {
            add(
                &mut findings,
                &template,
                "inconclusive",
                "One or both scenario captures are missing, duplicated or incomplete.",
            );
        }
        if base_repeat.is_some() && ar.is_none_or(|c| c.error.is_some()) {
            add(
                &mut findings,
                &template,
                "inconclusive",
                "Baseline repeat could not complete; stability is unverified.",
            );
        }
        if head_repeat.is_some() && br.is_none_or(|c| c.error.is_some()) {
            add(
                &mut findings,
                &template,
                "inconclusive",
                "Candidate repeat could not complete; stability is unverified.",
            );
        }
        for assertion in &scenario.assertions {
            let ac = one_check(a, &assertion.id);
            let bc = one_check(b, &assertion.id);
            let arc = one_check(ar, &assertion.id);
            let brc = one_check(br, &assertion.id);
            let intent = config
                .intended_changes
                .iter()
                .find(|i| i.scenario_id == scenario.id && i.assertion_id == assertion.id);
            let expected = intent.map(|i| &i.expected).unwrap_or(&assertion.expected);
            for (check, wanted) in [(ac, &assertion.expected), (bc, expected)] {
                match check {
                    None => coverage.not_run += 1,
                    Some(c) if c.error.is_some() => coverage.not_run += 1,
                    Some(c) if &c.actual == wanted => coverage.passed += 1,
                    _ => coverage.failed += 1,
                }
            }
            let mut detail = template.clone();
            detail.assertion_id = Some(assertion.id.clone());
            detail.base = Some(ac.map(|c| c.actual.clone()).unwrap_or(Value::Null));
            detail.head = Some(bc.map(|c| c.actual.clone()).unwrap_or(Value::Null));
            detail.expected = Some(expected.clone());
            let (Some(ac), Some(bc)) = (ac, bc) else {
                add(
                    &mut findings,
                    &detail,
                    "inconclusive",
                    "This assertion lacks observations on both states.",
                );
                continue;
            };
            if ac.error.is_some() || bc.error.is_some() {
                add(
                    &mut findings,
                    &detail,
                    "inconclusive",
                    "This assertion could not be evaluated in both states.",
                );
                continue;
            }
            if base_repeat.is_some()
                && arc.is_none_or(|c| c.error.is_some() || c.actual != ac.actual)
            {
                add(
                    &mut findings,
                    &detail,
                    "baseline-issue",
                    "The baseline assertion is unstable or its repeat is missing.",
                );
                continue;
            }
            if head_repeat.is_some()
                && brc.is_none_or(|c| c.error.is_some() || c.actual != bc.actual)
            {
                add(
                    &mut findings,
                    &detail,
                    "inconclusive",
                    "The candidate assertion did not repeat consistently.",
                );
                continue;
            }
            if ac.actual != assertion.expected {
                add(
                    &mut findings,
                    &detail,
                    "baseline-issue",
                    "The baseline already violates the original expectation; a new regression is not established.",
                );
                continue;
            }
            if &bc.actual != expected {
                add(
                    &mut findings,
                    &detail,
                    "regression",
                    "The baseline met its expectation and the candidate repeatedly violated its declared expectation. This does not establish dependency causality.",
                );
            } else if let Some(intent) = intent {
                if ac.actual != bc.actual {
                    detail.reason = Some(intent.reason.clone());
                    add(
                        &mut findings,
                        &detail,
                        "intended-change",
                        "The observed change matches the declared intended expectation.",
                    );
                }
            }
        }
        if scenario.screenshot {
            let mut stable = true;
            for (requested, first, repeat, label) in [
                (base_repeat.is_some(), a, ar, "Baseline"),
                (head_repeat.is_some(), b, br, "Candidate"),
            ] {
                if requested {
                    match images(
                        screenshot(first),
                        screenshot(repeat),
                        scenario.max_diff_pixel_ratio,
                    ) {
                        Visual::Same => (),
                        Visual::Different => {
                            stable = false;
                            add(
                                &mut findings,
                                &template,
                                if label == "Baseline" {
                                    "baseline-issue"
                                } else {
                                    "inconclusive"
                                },
                                &format!(
                                    "{label} screenshots changed between repeats; visual stability is unverified."
                                ),
                            );
                        }
                        Visual::Unavailable => {
                            stable = false;
                            add(
                                &mut findings,
                                &template,
                                "inconclusive",
                                &format!(
                                    "{label} repeat screenshot evidence is missing or unreadable."
                                ),
                            );
                        }
                    }
                }
            }
            match images(screenshot(a), screenshot(b), scenario.max_diff_pixel_ratio) {
                Visual::Unavailable => add(
                    &mut findings,
                    &template,
                    "inconclusive",
                    "Required screenshot evidence is missing or unreadable.",
                ),
                Visual::Different if stable => add(
                    &mut findings,
                    &template,
                    "needs-review",
                    "Screenshots differ beyond the configured tolerance. Pixels alone do not establish a regression or explain the whole image.",
                ),
                _ => (),
            }
        }
    }
    let unresolved = findings.iter().any(|f| {
        matches!(
            f.kind.as_str(),
            "inconclusive" | "needs-review" | "baseline-issue"
        )
    });
    let regression = findings.iter().any(|f| f.kind == "regression");
    Comparison {
        complete: !unresolved && coverage.not_run == 0,
        exit_code: if regression {
            1
        } else if unresolved || coverage.not_run > 0 {
            2
        } else {
            0
        },
        findings,
        coverage,
    }
}
