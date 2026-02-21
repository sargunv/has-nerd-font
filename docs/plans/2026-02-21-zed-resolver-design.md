# Zed Resolver & Terminal Detection Cleanup

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to
> implement this plan task-by-task.

**Goal:** Add a Zed config resolver that reads JSONC settings files to detect
Nerd Fonts, fix terminal detection gaps (Zed, Hyper), and remove dead code in
`from_term`.

**Architecture:** The Zed resolver reads `~/.config/zed/settings.json` (JSONC
format, same path on macOS and Linux) and optionally `<cwd>/.zed/settings.json`
for project overrides. It checks `terminal.font_family` first, falling back to
`buffer_font_family`. Uses `serde_jsonc` for JSONC parsing. Terminal detection
is updated to recognize `TERM_PROGRAM=zed` and `TERM_PROGRAM=Hyper`.

**Tech Stack:** Rust, serde_jsonc, insta (snapshot testing), assert_cmd

---

## Task 1: Add `serde_jsonc` dependency

**Files:**

- Modify: `Cargo.toml`

**Step 1: Add the dependency**

Add `serde_jsonc = "1"` to `[dependencies]` in `Cargo.toml`.

**Step 2: Verify it compiles**

Run: `cargo check` Expected: compiles successfully

**Step 3: Commit**

```
chore: add serde_jsonc dependency for JSONC parsing
```

---

## Task 2: Add Zed and Hyper to `from_term_program`, remove dead `wezterm` from `from_term`

**Files:**

- Modify: `src/terminal.rs:45-63`

**Step 1: Update `from_term_program` in `src/terminal.rs`**

Add two new match arms:

```rust
"zed" => Some(Terminal::Zed),
"hyper" => Some(Terminal::Hyper),
```

**Step 2: Remove dead code from `from_term`**

Remove the `"wezterm"` arm from `from_term()`. WezTerm sets
`TERM_PROGRAM=WezTerm` (caught by `from_term_program`), not `TERM=wezterm`. The
match arm is unreachable.

After removal, `from_term` should only match:

```rust
"xterm-ghostty" => Some(Terminal::Ghostty),
"xterm-kitty" => Some(Terminal::Kitty),
```

**Step 3: Verify it compiles**

Run: `cargo check` Expected: compiles successfully

**Step 4: Run existing tests**

Run: `cargo test` Expected: all existing tests pass (no test relies on
`TERM=wezterm` or `TERM_PROGRAM=zed/Hyper`)

**Step 5: Commit**

```
fix: add Zed and Hyper terminal detection, remove dead wezterm TERM match
```

---

## Task 3: Create Zed resolver module

**Files:**

- Create: `src/config/zed.rs`
- Modify: `src/config/mod.rs`

**Step 1: Create `src/config/zed.rs`**

The resolver should:

1. Accept `vars` (for `HOME`) and `cwd` (for project settings)
2. Try to find the effective font by: a. Read project settings from
   `<cwd>/.zed/settings.json` if it exists b. Read user settings from
   `$HOME/.config/zed/settings.json` c. Project settings font values override
   user settings font values d. Within merged settings, prefer
   `terminal.font_family` over `buffer_font_family`
3. Check the font name with `is_nerd_font`
4. Return a `DetectionResult`

Use typed deserialization:

```rust
#[derive(Deserialize)]
struct ZedSettings {
    #[serde(default)]
    terminal: Option<ZedTerminal>,
    #[serde(default)]
    buffer_font_family: Option<String>,
}

#[derive(Deserialize)]
struct ZedTerminal {
    #[serde(default)]
    font_family: Option<String>,
}
```

The resolver function signature should be:

```rust
pub fn resolve(vars: &[(String, String)], cwd: &Path) -> DetectionResult
```

Font resolution logic:

```rust
fn effective_font(project: &Option<ZedSettings>, user: &Option<ZedSettings>) -> Option<String> {
    // terminal.font_family from project settings
    // then terminal.font_family from user settings
    // then buffer_font_family from project settings
    // then buffer_font_family from user settings
    let terminal_font = project
        .as_ref()
        .and_then(|s| s.terminal.as_ref())
        .and_then(|t| t.font_family.clone())
        .or_else(|| {
            user.as_ref()
                .and_then(|s| s.terminal.as_ref())
                .and_then(|t| t.font_family.clone())
        });

    let buffer_font = project
        .as_ref()
        .and_then(|s| s.buffer_font_family.clone())
        .or_else(|| user.as_ref().and_then(|s| s.buffer_font_family.clone()));

    terminal_font.or(buffer_font)
}
```

Read each settings file with `serde_jsonc::from_str`. If the file doesn't exist,
treat it as `None` (not an error). If it exists but is malformed, return a
config error.

The `config_path` in the result should be whichever file the winning font came
from (project or user). If neither file sets a font but at least one file
exists, return a config error with reason "no font configured". If neither file
exists, return a config error with reason "no settings file found".

**Step 2: Wire it into `src/config/mod.rs`**

Add `mod zed;` and the match arm:

```rust
Terminal::Zed => zed::resolve(vars, _cwd),
```

Rename `_cwd` to `cwd` since it's now used.

**Step 3: Verify it compiles**

Run: `cargo check` Expected: compiles successfully

**Step 4: Commit**

```
feat: add Zed config resolver with JSONC settings parsing
```

---

## Task 4: Create Zed test fixtures

**Files:**

- Create: `tests/fixtures/zed/zed-default.jsonc`
- Create: `tests/fixtures/zed/zed-nerd-font-buffer.jsonc`
- Create: `tests/fixtures/zed/zed-nerd-font-terminal.jsonc`
- Create: `tests/fixtures/zed/zed-malformed.jsonc`

**Step 1: Create fixture files**

`tests/fixtures/zed/zed-default.jsonc`:

```jsonc
// Zed settings with default (non-Nerd) font
{ "buffer_font_family": "Monaco", "buffer_font_size": 14 }
```

`tests/fixtures/zed/zed-nerd-font-buffer.jsonc`:

```jsonc
// Zed settings with a Nerd Font as buffer font
{ "buffer_font_family": "JetBrainsMono Nerd Font", "buffer_font_size": 14 }
```

`tests/fixtures/zed/zed-nerd-font-terminal.jsonc`:

```jsonc
// Zed settings with a Nerd Font as terminal font
{
  "buffer_font_family": "Monaco",
  "terminal": { "font_family": "JetBrainsMono Nerd Font", "font_size": 14 },
}
```

`tests/fixtures/zed/zed-malformed.jsonc`:

```
this is not valid json at all {{{
```

**Step 2: Commit**

```
test: add Zed config fixture files
```

---

## Task 5: Add Zed test helper and integration tests

**Files:**

- Modify: `tests/support/mod.rs`
- Create: `tests/zed.rs`

**Step 1: Add `install_zed_fixture` to `tests/support/mod.rs`**

This helper is NOT cfg-gated (Zed works on macOS and Linux):

```rust
pub fn install_zed_fixture(home: &Path, fixture_name: &str) {
    let fixture_path = Path::new("tests")
        .join("fixtures")
        .join("zed")
        .join(fixture_name);
    let settings_path = home.join(".config/zed/settings.json");
    std::fs::create_dir_all(
        settings_path
            .parent()
            .expect("zed settings should have parent directory"),
    )
    .expect("failed to create zed settings directory");
    std::fs::copy(&fixture_path, &settings_path)
        .expect("failed to copy zed settings fixture");
}
```

Also add a non-cfg-gated `scenario_home` (the existing one is macOS-only). Or
remove the `#[cfg(target_os = "macos")]` gate on `scenario_home` since it will
now be needed on Linux too.

Add `install_zed_project_fixture` for project-level settings:

```rust
pub fn install_zed_project_fixture(cwd: &Path, fixture_name: &str) {
    let fixture_path = Path::new("tests")
        .join("fixtures")
        .join("zed")
        .join(fixture_name);
    let settings_path = cwd.join(".zed/settings.json");
    std::fs::create_dir_all(
        settings_path
            .parent()
            .expect("zed project settings should have parent directory"),
    )
    .expect("failed to create zed project settings directory");
    std::fs::copy(&fixture_path, &settings_path)
        .expect("failed to copy zed project settings fixture");
}
```

**Step 2: Create `tests/zed.rs` with integration tests**

Test scenarios:

- `zed_default` — buffer font is non-Nerd, exit code 6
- `zed_nerd_font_buffer` — buffer font is Nerd Font, exit code 0
- `zed_nerd_font_terminal` — terminal font is Nerd Font (overrides buffer), exit
  code 0
- `zed_malformed` — invalid JSONC, exit code 5
- `zed_project_override` — project settings override user settings

Each test follows the pattern from `iterm2_macos.rs`:

```rust
#[test]
fn zed_default_snapshots_json_and_explain() {
    let home = support::scenario_home("zed-default");
    support::install_zed_fixture(&home, "zed-default.jsonc");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "zed"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(6));
    assert_snapshot!("zed_default_json", support::stdout_json_snapshot(&output));
    assert_snapshot!("zed_default_explain", support::stderr_text(&output));
}
```

For the project override test, use `tempfile::tempdir()` for the cwd and pass it
to `run_cli`:

```rust
#[test]
fn zed_project_override_snapshots_json_and_explain() {
    let home = support::scenario_home("zed-project-override");
    support::install_zed_fixture(&home, "zed-default.jsonc");  // user: non-Nerd
    let cwd = tempfile::tempdir().expect("failed to create temp dir");
    support::install_zed_project_fixture(cwd.path(), "zed-nerd-font-buffer.jsonc");  // project: Nerd
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "zed"), ("HOME", &home_str)],
        Some(cwd.path()),
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!("zed_project_override_json", support::stdout_json_snapshot(&output));
    assert_snapshot!("zed_project_override_explain", support::stderr_text(&output));
}
```

**Step 3: Run tests and accept snapshots**

Run: `cargo test` (expect failures on first run — snapshot files don't exist
yet) Then: `mise run test:accept` to accept the new snapshots Then: `cargo test`
to confirm all pass

**Step 4: Verify snapshot content**

Manually inspect the generated snapshot files to confirm they contain the
expected values (correct terminal, font, config_path, etc.).

**Step 5: Commit**

```
test: add Zed resolver integration tests with snapshot assertions
```

---

## Task 6: Final verification

**Step 1: Run full test suite**

Run: `cargo test` Expected: all tests pass

**Step 2: Run lints**

Run: `cargo clippy` Expected: no warnings

**Step 3: Check formatting**

Run: `cargo fmt --check` Expected: no formatting issues

**Step 4: Commit any final fixes if needed**
