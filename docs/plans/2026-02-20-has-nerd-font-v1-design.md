# has-nerd-font v1 Design

## Goal

Build a top-down vertical slice of `has-nerd-font` that starts at the CLI
contract and flows through layered detection to a working `Terminal.app` config
resolver, while keeping extension points ready for additional terminal
resolvers.

## Scope for This Iteration

- Full vertical path: CLI -> `detect()` orchestrator -> layered checks -> config
  dispatcher -> `Terminal.app` resolver.
- Full env-var behavior (`NERD_FONT` truthy/falsy override, terminal
  identification signals, SSH markers).
- Terminal identity support in the registry and bundled short-circuit for
  Ghostty/WezTerm.
- Only `Terminal.app` implemented in Layer 3 for actual config resolution.
- Other known non-bundled terminals return `NoResolver` (exit 4) for now.

## Architecture

### `src/main.rs` (Top-level contract)

- Parse `--explain` and `--json`.
- Build runtime inputs (`vars`, `cwd`) and call `has_nerd_font::detect(...)`.
- Output behavior:
  - no flags: no output
  - `--json`: JSON to stdout
  - `--explain`: explanation to stderr
  - both: JSON stdout + explain stderr
- Exit with `result.exit_code()`.

### `src/lib.rs` (Detection orchestration)

- Run detection cascade in strict order:
  1. Env override
  2. Terminal identification + bundled terminal short-circuit
  3. Remote session gate
  4. Config resolver dispatch
- Use an internal layer-outcome pattern:
  - `Final(DetectionResult)` => stop
  - `Continue` => next layer

### `src/types.rs` (Canonical model and mapping)

- Define `DetectionResult`, `DetectionSource`, `Terminal`, `Confidence`.
- Centralize `exit_code()` mapping and `explain()` rendering.
- Keep serialization-ready fields for JSON output.

### `src/env.rs`

- Parse `NERD_FONT` with normalized truthy/falsy values.
- Outcomes:
  - truthy => definitive yes
  - falsy => definitive no (explicit disable)
  - unset/unrecognized => continue

### `src/terminal.rs`

- Resolve terminal identity with deterministic precedence:
  1. `TERM_PROGRAM`
  2. `TERM`
  3. terminal-specific env vars (`GHOSTTY_RESOURCES_DIR`, `KITTY_PID`,
     `WEZTERM_PANE`, `ITERM_SESSION_ID`, `VSCODE_*`, `ZED_TERM`, etc.)
- Handle bundled terminals (Ghostty, WezTerm) as immediate yes.
- If identification fails, return unknown terminal.

### `src/config/mod.rs`

- Dispatch by terminal enum.
- v1 implementation behavior:
  - `TerminalApp` => call `terminal_app` resolver
  - known-but-unimplemented resolvers => `NoResolver`

### `src/config/terminal_app.rs`

- Resolve config path from `HOME`:
  - `$HOME/Library/Preferences/com.apple.Terminal.plist`
- Parse plist, resolve default profile, extract profile font descriptor.
- Normalize extracted font and apply Nerd Font matching.
- Return positive/negative `TerminalConfig` result with metadata.
- Missing file, missing keys, parse failure, missing `HOME` => `ConfigError`.

## Data Model and Semantics

- `DetectionResult.detected`:
  - `Some(true)` for yes
  - `Some(false)` for no
  - `None` for unknown
- Source remains semantic; numeric exit code derived at boundary via mapping.
- `Confidence`:
  - default `Certain` in this iteration
  - keep `Probable` variant in model for future profile-any-match flows

## Exit Code Mapping

- `0`: `EnvVar`, `BundledTerminal`, or `TerminalConfig` with detection true
- `1`: `ExplicitDisable`
- `2`: `UnknownTerminal`
- `3`: `RemoteSession`
- `4`: `NoResolver`
- `5`: `ConfigError`
- `6`: `TerminalConfig` with detection false

## Layer Rules for This Iteration

1. Env override may terminate immediately.
2. Bundled terminals may terminate immediately.
3. If still unresolved and SSH markers are present, stop at remote-session
   unknown.
4. Config layer only runs when local and terminal is known.
5. Only `Terminal.app` does real config parsing in this iteration.

## Testing Strategy (Pragmatic, Vertical-first)

This project will prioritize high-value tests over trivial contract assertions.

### What to prioritize

- Integration tests that prove end-to-end vertical behavior and correct
  exit/source/output semantics.
- Logic tests where there is real branching risk:
  - env var parsing
  - terminal identification precedence
  - Nerd Font name matching normalization
  - Terminal.app plist extraction paths and error mapping

### What to avoid

- Low-value tests that only assert framework behavior or static boilerplate
  (`--help` output snapshots, placeholder text checks, etc.).

### TDD posture

- Prefer writing failing tests for each meaningful behavior change.
- Temporary exploratory/manual verification is acceptable during early vertical
  bring-up.
- Final state should retain only tests that enforce meaningful behavior and
  regressions.

## Milestone Slices

1. CLI contract skeleton + types + mapping helpers.
2. Wire layered orchestration (`detect`) top-down.
3. Implement env/terminal/remote layers.
4. Implement config dispatcher and `Terminal.app` resolver.
5. Add focused integration + core logic tests.
6. Polish explain/json output and error reasons.

## Deferred Work (Next Iterations)

- Add resolvers in this order: Kitty, Alacritty, iTerm2, VSCode, Zed, Hyper.
- Expand profile-aware confidence handling (`Probable`) where profile selection
  is ambiguous.
- Consider future layers only after v1 proves useful.
