use clap::Parser;
use has_nerd_font::detect;

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
        eprintln!("{}", result.explain());
    }

    std::process::exit(result.exit_code());
}
