fn main() {
    println!("cargo:rerun-if-env-changed=HAS_NERD_FONT_VERSION");
    if let Ok(version) = std::env::var("HAS_NERD_FONT_VERSION") {
        println!("cargo:rustc-env=CARGO_PKG_VERSION={version}");
    }
}
