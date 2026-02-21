# has-nerd-font v1 Vertical Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to
> implement this plan task-by-task.

**Goal:** Build a top-down `has-nerd-font` vertical slice from CLI through
layered detection to a working `Terminal.app` resolver, with meaningful
integration and logic tests.

**Architecture:** Start from CLI contract and `DetectionResult` model, then wire
a layered `detect()` orchestration that short-circuits by source. Implement env
parsing, terminal detection, and SSH gating before adding config dispatch and a
macOS `Terminal.app` plist resolver. Keep known-but-unimplemented resolvers
represented via `NoResolver` so additional terminals can be added without
refactoring the vertical flow.

**Tech Stack:** Rust (edition 2024), `clap`, `serde`, `serde_json`, `plist`
(macOS), `assert_cmd`, `tempfile`.

---

### Task 1: Bootstrap crate surface and dependencies

**Files:**

- Modify: `Cargo.toml`
- Modify: `src/main.rs`
- Create: `src/lib.rs`

**Step 1: Write the failing compile check for expected crate wiring**

Run: `cargo check`

Expected: FAIL after adding references in `src/main.rs` to missing library items
(e.g., unresolved import of `has_nerd_font::detect`).

**Step 2: Implement minimal crate wiring**

- Update `Cargo.toml` with:
  - `[lib] name = "has_nerd_font"`
  - `[[bin]] name = "has-nerd-font"`
  - dependencies: `clap` (derive), `serde` (derive), `serde_json`, `toml` (for
    future), and target-specific `plist` on macOS.
- Create `src/lib.rs` exporting a placeholder `detect(...)` and
  `DetectionResult` type re-export.
- Update `src/main.rs` to call into `detect(...)` (even if placeholder
  behavior).

**Step 3: Re-run compile check**

Run: `cargo check`

Expected: PASS.

**Step 4: Commit**

Run:
`git add Cargo.toml src/main.rs src/lib.rs && git commit -m "chore: scaffold library and binary entrypoints"`

Expected: commit created.

### Task 2: Define core types and exit/output mapping

**Files:**

- Create: `src/types.rs`
- Modify: `src/lib.rs`
- Modify: `src/main.rs`
- Test: `tests/detection_result_contract.rs`

**Step 1: Write failing tests for semantic mapping (high-value only)**

Create `tests/detection_result_contract.rs` with table-driven checks for:

- source/detected -> exit code mapping
- `--json` shape fields (at lib level via serialization)
- explain string includes key semantic signal

Run: `cargo test detection_result_contract -- --nocapture`

Expected: FAIL (types/methods not implemented).

**Step 2: Implement minimal core model**

- In `src/types.rs`, define:
  - `DetectionResult`
  - `DetectionSource`
  - `Terminal`
  - `Confidence`
- Implement:
  - `DetectionResult::exit_code()`
  - `DetectionResult::explain()`
- Ensure types are serde-serializable where needed for JSON output.

**Step 3: Hook types into CLI output path**

- Wire `src/main.rs` to format `--json` and `--explain` from `DetectionResult`.
- Keep no-output behavior when neither flag is provided.

**Step 4: Re-run tests**

Run: `cargo test detection_result_contract -- --nocapture`

Expected: PASS.

**Step 5: Commit**

Run:
`git add src/types.rs src/lib.rs src/main.rs tests/detection_result_contract.rs && git commit -m "feat: add detection result model and output mapping"`

Expected: commit created.

### Task 3: Add top-down detection orchestration skeleton

**Files:**

- Modify: `src/lib.rs`
- Create: `src/env.rs`
- Create: `src/terminal.rs`
- Create: `src/config/mod.rs`
- Test: `tests/detect_orchestration.rs`

**Step 1: Write failing orchestration tests**

Create `tests/detect_orchestration.rs` for sequence and short-circuit behavior:

- env override finalizes before later layers
- bundled terminal finalizes before SSH gate
- SSH gate finalizes before config dispatch

Run: `cargo test detect_orchestration -- --nocapture`

Expected: FAIL.

**Step 2: Implement orchestration and layer outcome enum**

- In `src/lib.rs`, add `detect(vars, cwd)` with strict ordered calls into:
  - `env`
  - `terminal`
  - remote gate
  - `config::resolve`
- Use an internal `LayerOutcome` (`Final`, `Continue`) to avoid using exit codes
  in flow control.

**Step 3: Add minimal stubs for layer modules**

- `src/env.rs`: return `Continue` for now.
- `src/terminal.rs`: return unknown/bundled placeholders as needed by tests.
- `src/config/mod.rs`: temporary `NoResolver` path.

**Step 4: Re-run tests**

Run: `cargo test detect_orchestration -- --nocapture`

Expected: PASS.

**Step 5: Commit**

Run:
`git add src/lib.rs src/env.rs src/terminal.rs src/config/mod.rs tests/detect_orchestration.rs && git commit -m "feat: wire layered detect orchestration"`

Expected: commit created.

### Task 4: Implement env var parsing layer

**Files:**

- Modify: `src/env.rs`
- Test: `tests/env_layer.rs`

**Step 1: Write failing env layer tests**

Create `tests/env_layer.rs` covering:

- `NERD_FONT=1/true/yes` -> definitive yes (`EnvVar`, exit 0)
- `NERD_FONT=0/false/no` -> definitive no (`ExplicitDisable`, exit 1)
- unset/unrecognized -> continue path (validated through `detect(...)` not
  finalizing at env layer)

Run: `cargo test env_layer -- --nocapture`

Expected: FAIL.

**Step 2: Implement parser**

- Normalize value: trim + lowercase.
- Match truthy/falsy tokens.
- Return `Continue` for unsupported tokens.

**Step 3: Re-run tests**

Run: `cargo test env_layer -- --nocapture`

Expected: PASS.

**Step 4: Commit**

Run:
`git add src/env.rs tests/env_layer.rs && git commit -m "feat: implement NERD_FONT override parsing"`

Expected: commit created.

### Task 5: Implement terminal identification and bundled short-circuit

**Files:**

- Modify: `src/terminal.rs`
- Modify: `src/types.rs`
- Test: `tests/terminal_layer.rs`

**Step 1: Write failing terminal layer tests**

Create `tests/terminal_layer.rs` for precedence and short-circuit semantics:

- `TERM_PROGRAM=ghostty` -> bundled yes
- `TERM=xterm-ghostty` -> bundled yes
- `TERM_PROGRAM=WezTerm` or `WEZTERM_PANE` -> bundled yes
- unknown/no signals -> `UnknownTerminal` (exit 2)
- representative known non-bundled identity (`Apple_Terminal`) resolves terminal
  enum and continues to later layers

Run: `cargo test terminal_layer -- --nocapture`

Expected: FAIL.

**Step 2: Implement detection precedence and terminal enum mapping**

- Check in order: `TERM_PROGRAM`, `TERM`, specific env vars.
- Return canonical enum variants (`Ghostty`, `WezTerm`, `TerminalApp`, etc.).
- Finalize immediately for bundled terminals.

**Step 3: Re-run tests**

Run: `cargo test terminal_layer -- --nocapture`

Expected: PASS.

**Step 4: Commit**

Run:
`git add src/terminal.rs src/types.rs tests/terminal_layer.rs && git commit -m "feat: add terminal detection and bundled terminal short-circuit"`

Expected: commit created.

### Task 6: Implement remote-session gate and no-resolver behavior

**Files:**

- Modify: `src/lib.rs`
- Modify: `src/config/mod.rs`
- Test: `tests/remote_and_dispatch.rs`

**Step 1: Write failing tests for remote and dispatch outcomes**

Create `tests/remote_and_dispatch.rs` for:

- known non-bundled terminal + `SSH_TTY` -> `RemoteSession` (exit 3)
- known non-bundled local terminal + unimplemented resolver -> `NoResolver`
  (exit 4)

Run: `cargo test remote_and_dispatch -- --nocapture`

Expected: FAIL.

**Step 2: Implement remote gate and dispatcher semantics**

- In `detect()`, remote gate runs after terminal identity and before config
  reads.
- In `config::resolve`, return `NoResolver` for all known terminals except
  `TerminalApp` (for now).

**Step 3: Re-run tests**

Run: `cargo test remote_and_dispatch -- --nocapture`

Expected: PASS.

**Step 4: Commit**

Run:
`git add src/lib.rs src/config/mod.rs tests/remote_and_dispatch.rs && git commit -m "feat: enforce remote gate and no-resolver fallback"`

Expected: commit created.

### Task 7: Implement Terminal.app resolver end-to-end

**Files:**

- Create: `src/config/terminal_app.rs`
- Modify: `src/config/mod.rs`
- Modify: `src/lib.rs`
- Test: `tests/terminal_app_resolver.rs`
- Test helper (optional): `tests/common/mod.rs`

**Step 1: Write failing Terminal.app resolver tests**

Create `tests/terminal_app_resolver.rs` with tempdir-backed `HOME`:

- plist with default profile using Nerd Font -> detected true (exit 0)
- plist with non-NF font -> detected false (exit 6)
- missing plist -> config error (exit 5)
- malformed plist -> config error (exit 5)
- missing `HOME` -> config error (exit 5)

Run: `cargo test terminal_app_resolver -- --nocapture`

Expected: FAIL.

**Step 2: Implement resolver**

- Path derivation strictly from `HOME`:
  - `$HOME/Library/Preferences/com.apple.Terminal.plist`
- Parse plist and read:
  - default profile name (`Default Window Settings`)
  - profile font descriptor in corresponding settings set
- Normalize font string and evaluate Nerd Font match.
- Return `DetectionResult` with metadata (`terminal`, `font`, `profile`,
  `config_path`).

**Step 3: Re-run tests**

Run: `cargo test terminal_app_resolver -- --nocapture`

Expected: PASS.

**Step 4: Commit**

Run:
`git add src/config/terminal_app.rs src/config/mod.rs src/lib.rs tests/terminal_app_resolver.rs tests/common/mod.rs && git commit -m "feat: add Terminal.app plist resolver"`

Expected: commit created.

### Task 8: Final CLI integration tests for meaningful vertical behavior

**Files:**

- Test: `tests/cli_integration.rs`
- Modify: `src/main.rs`

**Step 1: Write failing CLI integration tests (only meaningful cases)**

Create `tests/cli_integration.rs` using `assert_cmd` covering:

- no flags emits no output and correct exit code
- `--json` emits valid JSON with `exit_code`
- `--explain` emits stderr message
- both flags split output channels correctly
- one full vertical path using `TERM_PROGRAM=Apple_Terminal` + temp HOME plist
  fixture

Run: `cargo test cli_integration -- --nocapture`

Expected: FAIL.

**Step 2: Finish CLI behavior and fixture plumbing**

- Ensure stdout/stderr behavior exactly matches the contract.
- Ensure process exit code comes from `DetectionResult::exit_code()`.

**Step 3: Re-run targeted integration tests**

Run: `cargo test cli_integration -- --nocapture`

Expected: PASS.

**Step 4: Run full verification suite**

Run: `cargo test`

Expected: PASS.

**Step 5: Commit**

Run:
`git add tests/cli_integration.rs src/main.rs && git commit -m "test: verify end-to-end CLI vertical behavior"`

Expected: commit created.

### Task 9: Documentation touch-up for implemented subset

**Files:**

- Modify: `docs/has-nerd-font-design.md`

**Step 1: Write a failing documentation consistency check (manual)**

Run: `cargo test && cargo check`

Expected: PASS, but documentation still states broader resolver scope than
implemented v1 slice.

**Step 2: Update design doc with explicit current implementation status**

- Add a short "Current implementation status" subsection indicating only
  Terminal.app resolver is implemented in this iteration.

**Step 3: Re-run verification**

Run: `cargo test`

Expected: PASS.

**Step 4: Commit**

Run:
`git add docs/has-nerd-font-design.md && git commit -m "docs: clarify implemented resolver scope for v1 slice"`

Expected: commit created.

## Notes for Executor

- Use meaningful tests only; avoid low-value snapshots and framework-default
  behavior assertions.
- Preserve the top-down vertical: do not implement additional resolvers in this
  pass.
- Keep `NoResolver` paths explicit so future terminal resolvers drop in without
  changing orchestration.
- If platform-specific plist behavior requires conditional compilation, use
  `#[cfg(target_os = "macos")]` and provide deterministic `ConfigError` fallback
  on non-macOS.
