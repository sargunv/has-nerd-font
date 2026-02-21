use clap::Parser;
use has_nerd_font::detect;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long)]
    json: bool,
    #[arg(long)]
    explain: bool,
}

fn main() {
    let cli = Cli::parse();
    let env_vars: Vec<(String, String)> = std::env::vars().collect();
    let cwd = std::env::current_dir().expect("failed to get current working directory");
    let result = detect(&env_vars, &cwd);

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
