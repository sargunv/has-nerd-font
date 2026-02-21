# iTerm2 Default Profile Resolver Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to
> implement this plan task-by-task.

**Goal:** Add iTerm2 default-profile detection using shared plist/font helpers,
with minimal integration coverage and real-plist-derived fixtures.

**Architecture:** Keep `src/config/` terminal-only by adding
`src/config/iterm2.rs` and moving reusable logic into `src/plist.rs` and
`src/font.rs`. Reuse those helpers from both iTerm2 and Terminal.app resolvers.
Validate behavior through three macOS integration tests only: non-NF, NF, and
malformed plist.

**Tech Stack:** Rust (edition 2024), `plist`, `serde`/`serde_json`,
`assert_cmd`, `insta`.

---

### Task 1: Create iTerm2 fixtures from real plist shape

**Files:**

- Create: `tests/fixtures/iterm2/iterm2-real-default.plist`
- Create: `tests/fixtures/iterm2/iterm2-real-nerd-font.plist`
- Create: `tests/fixtures/iterm2/iterm2-malformed.plist`
- Modify: `tests/support/mod.rs`

**Step 1: Capture baseline default-profile plist from local machine**

Run:
`plutil -convert xml1 -o tests/fixtures/iterm2/iterm2-real-default.plist "$HOME/Library/Preferences/com.googlecode.iterm2.plist"`

Expected: fixture file created in XML plist format.

**Step 2: Redact fixture to parser-relevant keys only**

- Keep only:
  - `Default Bookmark Guid`
  - `New Bookmarks[*].Guid`
  - `New Bookmarks[*].Name` (optional)
  - `New Bookmarks[*].Normal Font`
- Remove personal/noise keys.

Expected: compact fixture still structurally matches iTerm2 parser path.

**Step 3: Ask user to temporarily switch iTerm2 default font to a Nerd Font**

Prompt user to change iTerm2 default profile font (for example to
`JetBrainsMono Nerd Font`) and keep the same profile selected as default.

Expected: user confirms font changed.

**Step 4: Capture Nerd Font variant fixture**

Run:
`plutil -convert xml1 -o tests/fixtures/iterm2/iterm2-real-nerd-font.plist "$HOME/Library/Preferences/com.googlecode.iterm2.plist"`

Expected: second fixture created and then redacted to same key set as Step 2.

**Step 5: Restore user font preference and create malformed fixture**

- Ask user to restore original font preference.
- Create malformed fixture by writing invalid plist bytes/text to
  `tests/fixtures/iterm2/iterm2-malformed.plist`.

Expected: malformed fixture cannot be parsed by plist loader.

**Step 6: Add fixture installer helper**

- Add `install_iterm2_fixture(home, fixture_name)` to `tests/support/mod.rs`
  that copies fixture to
  `$HOME/Library/Preferences/com.googlecode.iterm2.plist`.

Expected: tests can install iTerm2 fixtures the same way Terminal.app fixtures
are installed.

### Task 2: Add minimal iTerm2 integration tests

**Files:**

- Create: `tests/iterm2_macos.rs`

**Step 1: Write failing integration tests for expected outcomes only**

Create `tests/iterm2_macos.rs` (`#[cfg(target_os = "macos")]`) with 3 tests:

- default non-NF fixture -> exit `6`
- default NF fixture -> exit `0`
- malformed fixture -> exit `5`

Use `TERM_PROGRAM=iTerm.app`, `HOME=<scenario_home>`, `--json --explain`, and
snapshots for stdout/stderr.

**Step 2: Run targeted test and confirm failure**

Run: `cargo test --test iterm2_macos -- --nocapture`

Expected: FAIL (no iTerm2 resolver yet).

### Task 3: Implement iTerm2 resolver with shared helpers

**Files:**

- Create: `src/plist.rs`
- Create: `src/font.rs`
- Create: `src/config/iterm2.rs`
- Modify: `src/lib.rs`
- Modify: `src/config/mod.rs`
- Modify: `src/config/terminal_app.rs`

**Step 1: Refactor shared font logic into `src/font.rs`**

- Move Nerd Font matching and normalization helpers from
  `src/config/terminal_app.rs`.
- Keep behavior stable for existing Terminal.app tests.

Expected: no behavior change, shared API available for both resolvers.

**Step 2: Refactor shared plist logic into `src/plist.rs`**

- Add helpers for plist root dictionary loading and keyed-archive name decode.
- Replace duplicate plist traversal in `terminal_app.rs` where appropriate.

Expected: Terminal.app resolver still produces the same outputs.

**Step 3: Implement `src/config/iterm2.rs` (default profile only)**

- Resolve config path from `HOME`.
- Read `Default Bookmark Guid` and `New Bookmarks`.
- Find matching bookmark by `Guid`.
- Extract and normalize `Normal Font`.
- Return:
  - `TerminalConfig + Some(true)` on Nerd Font match
  - `TerminalConfig + Some(false)` on non-NF
  - `ConfigError` on missing/invalid data or malformed plist

Expected: `profile` populated from bookmark `Name` when available, otherwise
`None`.

**Step 4: Wire resolver dispatch**

- Update `src/config/mod.rs` to dispatch `Terminal::ITerm2` to
  `iterm2::resolve`.
- Ensure module declarations are in place and compile.

Expected: iTerm2 no longer returns `NoResolver`.

**Step 5: Run focused verification and update snapshots via test option**

Run:

- `cargo test --test iterm2_macos -- --nocapture`
- `INSTA_UPDATE=always cargo test --test iterm2_macos -- --nocapture`
- `cargo test --test terminal_app_macos -- --nocapture`

Expected: iTerm2 snapshots are generated/updated by the snapshot framework and
both suites pass.

**Step 6: Run full test suite**

Run: `cargo test`

Expected: PASS.
