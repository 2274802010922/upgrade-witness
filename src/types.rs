use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Assertion {
    pub id: String,
    pub selector: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub expected: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: String,
    pub selector: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}
impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: 1440,
            height: 900,
        }
    }
}
fn yes() -> bool {
    true
}
fn ratio() -> f64 {
    0.001
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Scenario {
    pub id: String,
    pub route: String,
    #[serde(default)]
    pub viewport: Viewport,
    #[serde(default)]
    pub actions: Vec<Action>,
    pub assertions: Vec<Assertion>,
    #[serde(default)]
    pub mask: Vec<String>,
    #[serde(default = "yes")]
    pub screenshot: bool,
    #[serde(default = "ratio")]
    pub max_diff_pixel_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Intent {
    pub scenario_id: String,
    pub assertion_id: String,
    pub expected: Value,
    pub reason: String,
}

fn dev() -> String {
    "dev".into()
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Project {
    #[serde(default = "dev")]
    pub start_script: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_script: Option<String>,
}
impl Default for Project {
    fn default() -> Self {
        Self {
            start_script: dev(),
            build_script: None,
        }
    }
}
fn timeout() -> u64 {
    15_000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WitnessConfig {
    pub schema_version: u32,
    #[serde(default)]
    pub project: Project,
    pub scenarios: Vec<Scenario>,
    #[serde(default)]
    pub intended_changes: Vec<Intent>,
    #[serde(default = "timeout")]
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Check {
    pub id: String,
    pub expected: Value,
    pub actual: Value,
    pub passed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScenarioCapture {
    pub scenario_id: String,
    pub checks: Vec<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screenshot_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub browser_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Finding {
    pub id: String,
    pub scenario_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion_id: Option<String>,
    pub kind: String,
    pub observation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coverage {
    pub planned: usize,
    pub passed: usize,
    pub failed: usize,
    pub not_run: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comparison {
    pub complete: bool,
    pub exit_code: i32,
    pub findings: Vec<Finding>,
    pub coverage: Coverage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
    pub id: String,
    pub path: String,
    pub sha256: String,
    pub media_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunReport {
    pub schema_version: String,
    pub tool_version: String,
    pub run_id: String,
    pub created_at: String,
    pub base_sha: String,
    pub head_sha: String,
    pub contract_sha256: String,
    pub config: WitnessConfig,
    pub environment: Value,
    pub complete: bool,
    pub exit_code: i32,
    pub findings: Vec<Finding>,
    pub evidence: Vec<Evidence>,
    pub coverage: Coverage,
    pub limitations: Vec<String>,
}
