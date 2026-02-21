use has_nerd_font::{DetectionResult, detect};

fn main() {
    let env_vars: Vec<(String, String)> = std::env::vars().collect();
    let cwd = std::env::current_dir().expect("failed to get current working directory");
    let _result: DetectionResult = detect(&env_vars, &cwd);
}
