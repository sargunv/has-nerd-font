# iTerm2 Default Profile Resolver Design

## Goal

Add the next vertical resolver slice for iTerm2 by reading only the active
default profile from `com.googlecode.iterm2.plist`, while extracting reusable
plist and font-matching logic into shared non-config modules.

## Scope

- Implement `iTerm2` resolver in Layer 3 using default profile only.
- Keep `src/config/` one-file-per-terminal.
- Move shared helpers out of `src/config/` into `src/plist.rs` and
  `src/font.rs`.
- Keep profile confidence as `Certain` (single resolved profile).
- No multi-profile any-match behavior.

## Architecture

### Resolver modules

- `src/config/mod.rs`
  - dispatch `Terminal::ITerm2` to `iterm2::resolve`
  - retain existing `Terminal.app` dispatch
- `src/config/iterm2.rs`
  - iTerm2-specific key traversal and result shaping only
  - no generic plist parsing logic
- `src/config/terminal_app.rs`
  - refactor to call shared `plist` and `font` helpers
  - preserve existing behavior

### Shared modules

- `src/plist.rs`
  - read plist root dictionary from path
  - decode keyed-archive font names where needed
  - provide safe dictionary/string lookup helpers with deterministic errors
- `src/font.rs`
  - normalize font strings (trim, optional quote cleanup, strip size suffix)
  - perform Nerd Font detection (`Nerd Font`, `NerdFont`, `NF`, `NFM`, `NFP`)

## Resolver behavior contract

Input path:

- `$HOME/Library/Preferences/com.googlecode.iterm2.plist`

Resolution flow:

1. Validate `HOME` is present and non-empty.
2. Load plist root dictionary.
3. Read non-empty `Default Bookmark Guid`.
4. Read `New Bookmarks` array.
5. Find bookmark with matching `Guid`.
6. Read `Normal Font` string from that bookmark.
7. Normalize font family and match with shared `font` logic.

Outputs:

- Match: `DetectionSource::TerminalConfig`, `detected: Some(true)`, exit `0`
- No match: `DetectionSource::TerminalConfig`, `detected: Some(false)`, exit `6`
- Any parse/lookup failure: `DetectionSource::ConfigError`, exit `5`

Error handling policy:

- Missing/invalid `Default Bookmark Guid` is `ConfigError`.
- Default GUID not found in bookmarks is `ConfigError`.
- Missing/invalid `Normal Font` for default bookmark is `ConfigError`.
- Malformed plist is `ConfigError`.

## Real-plist fixture strategy

Fixtures are derived from the user's real iTerm2 plist structure:

- Base fixture copied from real config shape.
- Irrelevant and personal keys removed.
- Keep only keys needed by parser realism:
  - `Default Bookmark Guid`
  - `New Bookmarks[*].Guid`
  - `New Bookmarks[*].Name` (optional metadata)
  - `New Bookmarks[*].Normal Font`

Two success fixtures come from real machine captures:

- One using current non-NF font.
- One after user temporarily changes iTerm2 default profile font to a Nerd Font.

## Testing scope

Minimal integration-only tests (no unit tests):

1. Non-NF default profile fixture -> exit `6`
2. NF default profile fixture -> exit `0`
3. Malformed plist fixture -> exit `5`

This keeps coverage focused on expected behavior while avoiding low-value
negative matrix expansion.

## Out of scope for this iteration

- Multi-profile any-match and `Confidence::Probable`
- Non-default profile fallback behavior
- Additional terminal resolvers beyond iTerm2
