use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::Command};
use upgrade_witness::{capture::launch_browser, report::redact};

#[derive(Parser)]
#[command(
    name = "uw",
    version,
    about = "Native Rust evidence for frontend upgrades"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Check Git and launch a trusted Chromium browser.
    Doctor,
    /// Verify two committed revisions. Target project scripts execute code.
    Verify {
        #[arg(long)]
        repo: PathBuf,
        #[arg(long)]
        base: String,
        #[arg(long)]
        head: String,
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long, required = true)]
        trust_project: bool,
    },
    /// Run the original synthetic Tailwind migration fixture.
    Demo {
        #[arg(long, default_value = "examples/tailwind-migration")]
        fixtures: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}
fn execute(cli: Cli) -> anyhow::Result<i32> {
    match cli.command {
        Commands::Doctor => {
            let mut git = Command::new("git");
            git.arg("--version");
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                git.creation_flags(0x08000000);
            }
            let git_ok = git.output().is_ok_and(|o| o.status.success());
            let browser = launch_browser();
            let browser_version = browser
                .as_ref()
                .ok()
                .and_then(|b| b.get_version().ok())
                .map(|v| v.product);
            let ready = git_ok && browser_version.is_some();
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &serde_json::json!({"version":env!("CARGO_PKG_VERSION"),"runtime":"native Rust","platform":std::env::consts::OS,"gitReady":git_ok,"browserReady":browser_version.is_some(),"browserVersion":browser_version,"ready":ready,"next":if ready{"Ready to verify trusted projects"}else{"Install Git/Chromium and set UW_BROWSER_EXECUTABLE if needed. Node/npm is needed only for the target frontend."}})
                )?
            );
            Ok(if ready { 0 } else { 2 })
        }
        Commands::Verify {
            repo,
            base,
            head,
            config,
            out,
            trust_project,
        } => {
            if !trust_project {
                anyhow::bail!("--trust-project is required");
            }
            let report = upgrade_witness::verify::verify(&repo, &base, &head, &config, &out)?;
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &serde_json::json!({"exitCode":report.exit_code,"complete":report.complete,"findings":report.findings.len(),"report":out.join("report.html")})
                )?
            );
            Ok(report.exit_code)
        }
        Commands::Demo { fixtures, out } => upgrade_witness::demo::run_demo(
            &fixtures,
            &out.unwrap_or_else(|| {
                PathBuf::from(".uw/demo-rust").join(uuid::Uuid::new_v4().to_string())
            }),
        ),
    }
}
fn main() {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let code = if error.use_stderr() { 64 } else { 0 };
            let _ = error.print();
            std::process::exit(code);
        }
    };
    let code = match execute(cli) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{}", redact(&error.to_string()));
            64
        }
    };
    std::process::exit(code);
}
