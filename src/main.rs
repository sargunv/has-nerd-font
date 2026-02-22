use clap::Parser;
use has_nerd_font::{DetectionResult, DetectionSource, detect};

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[arg(long)]
    json: bool,
    #[arg(long)]
    explain: bool,
}

fn main() {
    let cli = Cli::parse();
    let env_vars: Vec<(String, String)> = std::env::vars().collect();
    let result = detect(&env_vars);

    if cli.json {
        println!(
            "{}",
            serde_json::to_string(&result).expect("failed to serialize result as json")
        );
    }

    if cli.explain {
        eprintln!("{}", explain(&result));
    }

    std::process::exit(exit_code(&result));
}

fn exit_code(result: &DetectionResult) -> i32 {
    match (&result.source, result.detected) {
        (DetectionSource::EnvVar, _) => 0,
        (DetectionSource::BundledTerminal, _) => 0,
        (DetectionSource::ExplicitDisable, _) => 1,
        (DetectionSource::UnknownTerminal, _) => 2,
        (DetectionSource::RemoteSession, _) => 3,
        (DetectionSource::NoResolver, _) => 4,
        (DetectionSource::ConfigError, _) => 5,
        (DetectionSource::TerminalConfig, Some(true)) => 0,
        (DetectionSource::TerminalConfig, Some(false)) => 6,
        (DetectionSource::TerminalConfig, None) => 5,
        _ => 1,
    }
}

fn explain(result: &DetectionResult) -> String {
    match &result.source {
        DetectionSource::EnvVar => "detected Nerd Font from NERD_FONT override".to_string(),
        DetectionSource::ExplicitDisable => {
            "Nerd Font explicitly disabled by NERD_FONT override".to_string()
        }
        DetectionSource::UnknownTerminal => {
            "cannot determine terminal; terminal is unknown".to_string()
        }
        DetectionSource::RemoteSession => {
            "running in remote session; local terminal config not inspected".to_string()
        }
        DetectionSource::NoResolver => "known terminal has no resolver implemented yet".to_string(),
        DetectionSource::ConfigError => format!(
            "failed to read terminal configuration: {}",
            result.error_reason.as_deref().unwrap_or("unknown reason")
        ),
        DetectionSource::BundledTerminal => {
            "terminal ships with Nerd Font support by default".to_string()
        }
        DetectionSource::TerminalConfig => {
            if result.detected == Some(true) {
                "terminal configuration indicates a Nerd Font is active".to_string()
            } else if result.detected == Some(false) {
                "terminal configuration does not indicate a Nerd Font".to_string()
            } else {
                "terminal configuration status is unknown".to_string()
            }
        }
        _ => "unknown detection source".to_string(),
    }
}
