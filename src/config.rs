use crate::types::WitnessConfig;
use anyhow::{Result, bail};
use regex::Regex;
use std::{collections::HashSet, fs, path::Path};

pub fn parse_config(text: &str) -> Result<WitnessConfig> {
    let config: WitnessConfig = serde_json::from_str(text)?;
    let id = Regex::new(r"^[A-Za-z0-9][A-Za-z0-9._-]{0,79}$")?;
    let script = Regex::new(r"^[A-Za-z0-9_:.-]+$")?;
    if config.schema_version != 1 {
        bail!("schemaVersion must be 1");
    }
    if !(1000..=120000).contains(&config.timeout_ms) {
        bail!("timeoutMs must be 1000..120000");
    }
    if !script.is_match(&config.project.start_script)
        || config
            .project
            .build_script
            .as_ref()
            .is_some_and(|s| !script.is_match(s))
    {
        bail!("Invalid npm script name");
    }
    if config.scenarios.is_empty() || config.scenarios.len() > 30 {
        bail!("Expected 1..30 scenarios");
    }
    let mut scenario_ids = HashSet::new();
    for scenario in &config.scenarios {
        if !id.is_match(&scenario.id) || !scenario_ids.insert(&scenario.id) {
            bail!("Scenario ids must be valid and unique");
        }
        if !scenario.route.starts_with('/')
            || scenario.route.starts_with("//")
            || scenario.route.contains('\\')
        {
            bail!("Routes must be same-origin paths");
        }
        if !(320..=2560).contains(&scenario.viewport.width)
            || !(200..=2160).contains(&scenario.viewport.height)
        {
            bail!("Invalid viewport");
        }
        if !(0.0..=1.0).contains(&scenario.max_diff_pixel_ratio) {
            bail!("Invalid pixel difference ratio");
        }
        if scenario.assertions.is_empty()
            || scenario.assertions.len() > 50
            || scenario.actions.len() > 30
            || scenario.mask.len() > 50
        {
            bail!("Scenario exceeds supported limits");
        }
        let mut assertion_ids = HashSet::new();
        for a in &scenario.assertions {
            if !id.is_match(&a.id)
                || !assertion_ids.insert(&a.id)
                || a.selector.is_empty()
                || a.selector.len() > 500
            {
                bail!("Invalid or duplicate assertion");
            }
            match a.kind.as_str() {
                "visible" if a.expected.is_boolean() => (),
                "text" if a.expected.is_string() => (),
                "attribute" | "style"
                    if a.expected.is_string() && a.name.as_ref().is_some_and(|n| !n.is_empty()) => {
                }
                _ => bail!("Invalid assertion kind or expected value"),
            }
        }
        for a in &scenario.actions {
            if a.selector.is_empty() || a.selector.len() > 500 {
                bail!("Invalid action selector");
            }
            match a.action_type.as_str() {
                "click" => (),
                "fill" | "press" if a.value.is_some() => (),
                _ => bail!("Invalid action"),
            }
        }
        if scenario.mask.iter().any(|m| m.is_empty()) {
            bail!("Mask selectors cannot be empty");
        }
    }
    let mut intents = HashSet::new();
    for intent in &config.intended_changes {
        let assertion = config
            .scenarios
            .iter()
            .find(|s| s.id == intent.scenario_id)
            .and_then(|s| s.assertions.iter().find(|a| a.id == intent.assertion_id));
        let Some(assertion) = assertion else {
            bail!("Unknown intended-change assertion");
        };
        if intent.expected.is_boolean() != assertion.expected.is_boolean()
            || intent.expected.is_string() != assertion.expected.is_string()
            || intent.reason.chars().count() < 10
            || !intents.insert((&intent.scenario_id, &intent.assertion_id))
        {
            bail!("Invalid or duplicate intended change");
        }
    }
    Ok(config)
}

pub fn load_config(path: &Path) -> Result<WitnessConfig> {
    parse_config(&fs::read_to_string(path)?)
}
