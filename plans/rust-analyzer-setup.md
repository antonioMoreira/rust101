# Plan: Configure Rust Analyzer and Clippy

## Objective
Verify the `rust-analyzer` setup in VS Code and configure it to use `cargo clippy` as the default linting tool for real-time feedback.

## Key Files & Context
- `.vscode/settings.json` (to be created)
- `src/main.rs` (for testing the configuration)

## Implementation Steps
1. Create a new file at `.vscode/settings.json` in the workspace root.
2. Add the following configuration to explicitly set `clippy` as the check command:
   ```json
   {
       "rust-analyzer.check.command": "clippy"
   }
   ```

## Verification & Testing
1. **Verify Extension:** Open VS Code, open the Extensions view (`Ctrl+Shift+X`), and ensure the `rust-analyzer` extension by `rust-lang` is installed and enabled.
2. **Reload Server:** Reload the VS Code window or use the Command Palette to run `Rust Analyzer: Restart server` so the new settings take effect.
3. **Test Clippy Feedback:** Temporarily add non-idiomatic code to `src/main.rs` to see if Clippy warns you. For example, add an unnecessary clone on a primitive type:
   ```rust
   fn main() {
       let x: i32 = 5;
       let y = x.clone(); // Clippy should warn about this: `clone_on_copy`
       println!("Hello, world! {y}");
   }
   ```
4. **Observe:** Check that VS Code displays a warning (yellow squiggle) under `.clone()` and that the hover information specifically mentions `clippy(clone_on_copy)`.