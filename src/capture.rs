use crate::types::*;
use anyhow::{Context, Result, bail};
use base64::Engine;
use headless_chrome::{
    Browser, LaunchOptionsBuilder, Tab, browser::tab::ModifierKey, protocol::cdp::types::Method,
};
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    thread,
    time::{Duration, Instant},
};

macro_rules! cdp {
    ($name:ident,$method:literal) => {
        #[derive(Debug, Serialize)]
        struct $name {
            #[serde(flatten)]
            params: Value,
        }
        impl Method for $name {
            const NAME: &'static str = $method;
            type ReturnObject = Value;
        }
    };
}
cdp!(Evaluate, "Runtime.evaluate");
cdp!(Metrics, "Emulation.setDeviceMetricsOverride");
cdp!(Locale, "Emulation.setLocaleOverride");
cdp!(Timezone, "Emulation.setTimezoneOverride");
cdp!(Media, "Emulation.setEmulatedMedia");
cdp!(ServiceWorkers, "Network.setBypassServiceWorker");
cdp!(UserAgent, "Network.setUserAgentOverride");
cdp!(Screenshot, "Page.captureScreenshot");

pub fn launch_browser() -> Result<Browser> {
    let executable = env::var_os("UW_BROWSER_EXECUTABLE").map(PathBuf::from);
    let args: Vec<&OsStr> = vec![
        OsStr::new("--lang=en-US"),
        OsStr::new("--disable-background-networking"),
        OsStr::new("--no-first-run"),
    ];
    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .path(executable)
        .args(args)
        .idle_browser_timeout(Duration::from_secs(120))
        .build()?;
    Browser::new(options)
        .context("Could not launch Chromium; set UW_BROWSER_EXECUTABLE to a trusted browser binary")
}

fn eval(tab: &Tab, expression: String, timeout: u64) -> Result<Value> {
    let result=tab.call_method(Evaluate{params:json!({"expression":expression,"returnByValue":true,"awaitPromise":true,"timeout":timeout,"userGesture":false})})?;
    if result.get("exceptionDetails").is_some() {
        bail!("Page evaluation failed");
    }
    Ok(result["result"]["value"].clone())
}
fn until(tab: &Tab, expression: String, timeout: u64) -> Result<()> {
    let deadline = Instant::now() + Duration::from_millis(timeout);
    loop {
        if eval(tab, expression.clone(), timeout)?.as_bool() == Some(true) {
            return Ok(());
        }
        if Instant::now() >= deadline {
            bail!("Page readiness timed out");
        }
        thread::sleep(Duration::from_millis(50));
    }
}
fn check(tab: &Tab, assertion: &Assertion, expected: &Value, timeout: u64) -> Check {
    let data = json!({"selector":assertion.selector,"kind":assertion.kind,"name":assertion.name});
    let expression = format!(
        r#"(()=>{{const a={data};const matches=document.querySelectorAll(a.selector);if(matches.length>1)throw new Error('ambiguous');const el=matches[0];if(!el)return a.kind==='visible'?false:null;switch(a.kind){{case 'visible':{{const r=el.getBoundingClientRect();return r.width>0&&r.height>0&&getComputedStyle(el).visibility!=='hidden'&&getComputedStyle(el).visibility!=='collapse';}}case 'text':return (el.textContent||'').trim();case 'attribute':return el.getAttribute(a.name);case 'style':return getComputedStyle(el).getPropertyValue(a.name).trim();default:throw new Error('unsupported')}}}})()"#
    );
    let deadline = Instant::now() + Duration::from_millis(timeout);
    loop {
        match eval(tab, expression.clone(), timeout) {
            Ok(actual) => {
                let passed = actual == *expected;
                if passed || Instant::now() >= deadline {
                    return Check {
                        id: assertion.id.clone(),
                        expected: expected.clone(),
                        actual,
                        passed,
                        error: None,
                    };
                }
            }
            Err(_) => {
                return Check {
                    id: assertion.id.clone(),
                    expected: expected.clone(),
                    actual: Value::Null,
                    passed: false,
                    error: Some("Assertion evaluation failed; page details omitted".into()),
                };
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}

pub fn capture_scenarios(
    base_url: &str,
    config: &WitnessConfig,
    out: &Path,
    label: &str,
) -> Vec<ScenarioCapture> {
    let initialize = (|| -> Result<Browser> {
        fs::create_dir_all(out)?;
        launch_browser()
    })();
    let browser = match initialize {
        Ok(b) => b,
        Err(_) => {
            return config
                .scenarios
                .iter()
                .map(|s| ScenarioCapture {
                    scenario_id: s.id.clone(),
                    checks: vec![],
                    screenshot_path: None,
                    error: Some("Browser or output initialization failed; run uw doctor".into()),
                    browser_version: "unavailable".into(),
                })
                .collect();
        }
    };
    let version = browser
        .get_version()
        .map(|v| v.product)
        .unwrap_or_else(|_| "unknown".into());
    let mut captures = vec![];
    for scenario in &config.scenarios {
        let mut capture = ScenarioCapture {
            scenario_id: scenario.id.clone(),
            checks: vec![],
            screenshot_path: None,
            error: None,
            browser_version: version.clone(),
        };
        let mut stage = "Scenario initialization";
        let result = (|| -> Result<()> {
            let context = browser.new_context()?;
            let tab = context.new_tab()?;
            tab.set_default_timeout(Duration::from_millis(config.timeout_ms));
            tab.call_method(Metrics{params:json!({"width":scenario.viewport.width,"height":scenario.viewport.height,"deviceScaleFactor":1,"mobile":false})})?;
            tab.call_method(Locale {
                params: json!({"locale":"en-US"}),
            })?;
            tab.call_method(Timezone {
                params: json!({"timezoneId":"UTC"}),
            })?;
            tab.call_method(Media{params:json!({"features":[{"name":"prefers-reduced-motion","value":"reduce"},{"name":"prefers-color-scheme","value":"light"}]})})?;
            tab.call_method(ServiceWorkers {
                params: json!({"bypass":true}),
            })?;
            let user_agent = browser.get_version()?.user_agent;
            tab.call_method(UserAgent {
                params: json!({"userAgent":user_agent,"acceptLanguage":"en-US"}),
            })?;
            let base = url::Url::parse(base_url)?;
            let target = base.join(&scenario.route)?;
            if base.origin() != target.origin() {
                bail!("Cross-origin scenario rejected");
            }
            stage = "Page navigation";
            tab.navigate_to(target.as_str())?;
            until(
                &tab,
                "document.readyState==='complete'&&document.fonts.status==='loaded'".into(),
                config.timeout_ms,
            )?;
            let final_url = tab.get_url();
            if url::Url::parse(&final_url)?.origin() != base.origin() {
                bail!("Cross-origin redirect rejected");
            }
            let status = eval(
                &tab,
                "performance.getEntriesByType('navigation')[0]?.responseStatus".into(),
                config.timeout_ms,
            )?;
            if !status.as_u64().is_some_and(|s| (200..400).contains(&s)) {
                bail!("Navigation status failed or unavailable");
            }
            // Disable visual animation consistently before actions and measurements.
            eval(&tab,"(()=>{const s=document.createElement('style');s.textContent='*,*::before,*::after{animation:none!important;transition:none!important;caret-color:transparent!important}';document.documentElement.appendChild(s);return true})()".into(),config.timeout_ms)?;
            for action in &scenario.actions {
                stage = "Scenario action";
                let selector = serde_json::to_string(&action.selector)?;
                until(
                    &tab,
                    format!("document.querySelectorAll({selector}).length===1"),
                    config.timeout_ms,
                )?;
                let element = tab.wait_for_element(&action.selector)?;
                match action.action_type.as_str() {
                    "click" => {
                        element.click()?;
                    }
                    "fill" => {
                        element.click()?;
                        tab.press_key_with_modifiers("a", Some(&[ModifierKey::Ctrl]))?;
                        tab.press_key("Backspace")?;
                        tab.type_str(action.value.as_deref().unwrap_or(""))?;
                    }
                    "press" => {
                        element.call_js_fn("function(){this.focus();}", vec![], false)?;
                        tab.press_key(action.value.as_deref().unwrap_or(""))?;
                    }
                    _ => bail!("Unsupported action"),
                }
            }
            stage = "Assertion collection";
            for assertion in &scenario.assertions {
                let intent = if label == "head" || label == "head-repeat" {
                    config
                        .intended_changes
                        .iter()
                        .find(|i| i.scenario_id == scenario.id && i.assertion_id == assertion.id)
                } else {
                    None
                };
                capture.checks.push(check(
                    &tab,
                    assertion,
                    intent.map(|i| &i.expected).unwrap_or(&assertion.expected),
                    config.timeout_ms,
                ));
            }
            if scenario.screenshot {
                stage = "Masked screenshot";
                let masks = serde_json::to_string(&scenario.mask)?;
                let expression = format!(
                    r#"(()=>{{const selectors=['input:not([type="button"]):not([type="submit"]):not([type="reset"]),textarea,[contenteditable]:not([contenteditable="false"]),[data-uw-sensitive]',...{masks}];const targets=[...new Set(selectors.flatMap(selector=>[...document.querySelectorAll(selector)]))];const rects=targets.map(el=>{{const r=el.getBoundingClientRect();return {{x:r.left+scrollX,y:r.top+scrollY,width:r.width,height:r.height}};}});for(const el of targets){{for(const node of [el,...el.querySelectorAll('*')])node.style.setProperty('visibility','hidden','important');}}return {{width:Math.max(innerWidth,document.documentElement.scrollWidth),height:Math.max(innerHeight,document.documentElement.scrollHeight),rects}};}})()"#
                );
                let size = eval(&tab, expression, config.timeout_ms)?;
                let width = size["width"].as_u64().context("Missing screenshot width")?;
                let height = size["height"]
                    .as_u64()
                    .context("Missing screenshot height")?;
                if width > 4096 || height > 20000 {
                    bail!("Screenshot exceeds supported dimensions");
                }
                let result=tab.call_method(Screenshot{params:json!({"format":"png","fromSurface":true,"captureBeyondViewport":true,"clip":{"x":0,"y":0,"width":width,"height":height,"scale":1}})})?;
                let png = base64::prelude::BASE64_STANDARD
                    .decode(result["data"].as_str().context("Missing screenshot")?)?;
                // Hide elements before capture, then paint opaque rectangles in memory.
                // Pixel masking also covers modal/popover top layers; raw bytes are never saved.
                let mut bitmap =
                    image::load_from_memory_with_format(&png, image::ImageFormat::Png)?.to_rgba8();
                for rect in size["rects"]
                    .as_array()
                    .context("Missing screenshot masks")?
                {
                    let x = rect["x"].as_f64().context("Invalid mask x")?;
                    let y = rect["y"].as_f64().context("Invalid mask y")?;
                    let w = rect["width"].as_f64().context("Invalid mask width")?;
                    let h = rect["height"].as_f64().context("Invalid mask height")?;
                    let left = x.floor().max(0.0).min(f64::from(bitmap.width())) as u32;
                    let top = y.floor().max(0.0).min(f64::from(bitmap.height())) as u32;
                    let right = (x + w).ceil().max(0.0).min(f64::from(bitmap.width())) as u32;
                    let bottom = (y + h).ceil().max(0.0).min(f64::from(bitmap.height())) as u32;
                    for row in top..bottom {
                        for column in left..right {
                            bitmap.put_pixel(column, row, image::Rgba([0, 0, 0, 255]));
                        }
                    }
                }
                let path = out.join(format!("{}.png", scenario.id));
                bitmap.save(&path)?;
                capture.screenshot_path =
                    Some(fs::canonicalize(path)?.to_string_lossy().into_owned());
            }
            tab.close(true)?;
            Ok(())
        })();
        if result.is_err() {
            capture.error = Some(format!(
                "{stage} could not complete; page/process details omitted"
            ));
        }
        captures.push(capture);
    }
    captures
}
