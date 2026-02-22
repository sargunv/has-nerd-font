# AGENTS.md

## Project Overview

`has-nerd-font` is a Rust CLI tool that detects whether the current terminal
session can render Nerd Font glyphs. It inspects environment variables and
terminal config files to determine font status.

## Build Commands

```sh
mise run build               # Debug build
mise run build:release       # Release build
mise run run                 # Run the CLI
```

## Test Commands

```sh
mise run test                          # Run all tests
mise run test <name>                   # Run tests matching a name filter
mise run test --file <file>            # Run all tests in a file (e.g., alacritty)
mise run test --file <file> <name>     # Combine file and name filter
```

Examples of running a single test:

```sh
mise run test alacritty_nerd_font_snapshots_json_and_explain
mise run test --file vscode vscode_nerd_font_terminal_snapshots_json_and_explain
```

### Snapshot Testing

Tests use `insta` for snapshot testing. Snapshots live in `tests/snapshots/`.

```sh
mise run test:accept             # Accept all snapshot changes
```

`test:accept` supports the same `--file` and `<name>` arguments as `test`.

When adding or modifying behavior, update snapshots with `mise run test:accept`
and review the diffs before committing.

### Platform-Specific Tests

`tests/iterm2_macos.rs` and `tests/terminal_app_macos.rs` are gated with
`#![cfg(target_os = "macos")]` — they compile to nothing on Linux. CI runs tests
on both `ubuntu-latest` and `macos-latest`.

## Lint and Format Commands

```sh
mise run check               # Run all linters and formatters (check mode)
mise run fix                 # Run all linters and formatters (fix mode)
```

CI runs `mise run fix` and then fails if any files were modified, so always run
`mise run fix` before committing.

## Code Style

### Error Handling

- **No `unwrap()` or `panic!()` in library code.** Errors are represented as
  `DetectionResult` with `source: DetectionSource::ConfigError` and
  `error_reason: Some(reason)`.
- Internal fallible functions return `Result<T, String>` with human-readable
  error messages.
- `expect("descriptive message")` is acceptable in `main.rs` for truly fatal
  errors and in test code.
- Config file parsing returns three-way results: `Ok(Some(data))` / `Ok(None)`
  (not found) / `Err(reason)`.

### Conditional Compilation

macOS-specific code is gated with `#[cfg(target_os = "macos")]` at both module
and function level. Non-macOS platforms provide stub implementations that return
config errors. Test files for macOS use file-level
`#![cfg(target_os = "macos")]`.

## Architecture

Detection follows a layered pipeline (`LayerOutcome<T>` enum in `src/lib.rs`):

1. **env_layer** — check `NERD_FONT` env var override
2. **terminal_layer** — identify terminal (bundled / identified / unknown)
3. **ssh_gate_layer** — block detection in remote SSH sessions
4. **config::resolve** — dispatch to terminal-specific config reader

Each layer returns `LayerOutcome::Final(result)` to short-circuit or
`LayerOutcome::Continue(data)` to pass data forward.

## Test Patterns

All tests are integration tests in `tests/*.rs`. Each test file includes
`mod support;` referencing `tests/support/mod.rs`. No unit tests in `src/`.

Standard test structure:

1. Create isolated home with `support::scenario_home("name")`
2. Install fixture files (e.g., `support::install_alacritty_fixture`)
3. Run CLI with `support::run_cli(args, env_vars, optional_cwd)` — this calls
   `env_clear()` then sets only the given env vars
4. Assert exit code: `assert_eq!(output.status.code(), Some(N))`
5. Assert JSON snapshot:
   `assert_snapshot!("name", support::stdout_json_snapshot(&output))`
6. Assert explain snapshot:
   `assert_snapshot!("name", support::stderr_text(&output))`

Test names follow: `{terminal}_{scenario}_snapshots_json_and_explain`

Fixtures live in `tests/fixtures/{terminal}/`. Snapshots are normalized — temp
directory paths are replaced with `<SCENARIO_HOME>`.

## Tool Versions (managed by mise)

Install all tools: `mise install`

## CI

- **Lint job** (ubuntu): `mise run fix`, then fail if any files changed
- **Test job** (ubuntu + macos matrix): `mise run test`

Pre-commit hook runs `hk run pre-commit` via mise. Disable with `HK=0`.
