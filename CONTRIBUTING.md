# Contributing to Lucide Icons Extension

Thank you for your interest in contributing! This guide will help you get started.

## Development Setup

### Prerequisites

- **Rust** (installed via [rustup](https://rustup.rs/))
- **Cargo** (comes with Rust)
- **wasm32-wasip1 target**: `rustup target add wasm32-wasip1`
- **Zed Editor** (for testing)

### Local Development

1. **Clone the repository**

   ```bash
   git clone https://github.com/Michael-Obele/lucide-icons.git
   cd lucide-icons
   ```

2. **Build the extension**

   ```bash
   cargo build --release
   ```

3. **Install as dev extension in Zed**

   - Open Zed
   - Press `Cmd+K` (or `Ctrl+K`)
   - Type `zed: install dev extension`
   - Select this project folder

4. **Test the extension**
   - Open Zed Assistant (`Cmd+J` / `Ctrl+J`)
   - Try slash commands: `/lucide-search home`, `/lucide-browse`
   - Check logs: `zed: open log`

### Debug Logging

For verbose output:

```bash
zed --foreground
```

This shows `println!` and `dbg!` output from the extension.

## Project Structure

```
lucide-icons/
├── extension.toml          # Extension metadata
├── Cargo.toml              # Rust dependencies
├── src/
│   └── lib.rs              # Main extension code
├── lucide-icons/
│   ├── icons-data.json     # Icon database
│   └── src/                # Icon data source
└── README.md               # User documentation
```

## Code Guidelines

### Rust Code

- Follow Rust conventions (`cargo fmt`)
- Add comments for complex logic
- Handle errors gracefully with `Result<T, String>`

### Slash Commands

All slash commands are defined in:

- `extension.toml` (command registration)
- `src/lib.rs` (implementation in `run_slash_command()`)

### Icon Data

- Icons loaded from `lucide-icons/icons-data.json`
- Data structure: `{ name, keywords, categories }`
- Keep icon names in kebab-case (e.g., `user-circle`)

## Making Changes

### Adding a New Slash Command

1. **Register in `extension.toml`**:

   ```toml
   [slash_commands]
   my-command = { description = "My cool command", requires_argument = false }
   ```

2. **Implement in `src/lib.rs`**:

   ```rust
   fn run_slash_command(
       &self,
       command: SlashCommand,
       // ...
   ) -> Result<SlashCommandOutput> {
       match command.name.as_str() {
           "my-command" => {
               // Implementation
           }
           // ... other commands
       }
   }
   ```

3. **Test the command** in Zed Assistant

### Updating Icon Data

1. Update `lucide-icons/icons-data.json`
2. Rebuild: `cargo build --release`
3. Reload extension in Zed

## Testing

### Manual Testing Checklist

- [ ] `/lucide-search <icon>` returns results
- [ ] `/lucide-browse` shows categories
- [ ] Framework commands work (react, svelte, vue, html)
- [ ] Tab completion suggests icons
- [ ] Error messages are helpful
- [ ] Code examples are correct

### Edge Cases to Test

- Invalid icon names
- Empty search queries
- Special characters in arguments
- Very long icon names

## Pull Request Process

1. **Fork the repository**
2. **Create a feature branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**

   - Follow code guidelines
   - Test thoroughly
   - Update README if needed

4. **Commit with clear messages**

   See [.github/COMMIT_CONVENTIONS.md](.github/COMMIT_CONVENTIONS.md) for detailed commit message guidelines.

   Basic format:

   ```bash
   git commit -m "Add: description of feature"
   # or
   git commit -m "Fix: description of bug fix"
   ```

   **Version Bumping (Optional):**
   To trigger an automatic version bump when merged to `main`, include one of these tags:

   - `[patch]` — Bug fixes (0.0.1 → 0.0.2)
   - `[minor]` — New features (0.0.1 → 0.1.0)
   - `[major]` — Breaking changes (0.0.1 → 1.0.0)

   Example:

   ```bash
   git commit -m "Add new slash command for icon categories [minor]"
   ```

5. **Push and open a PR**

   ```bash
   git push origin feature/your-feature-name
   ```

6. **In your PR description, include:**
   - What changed and why
   - How to test the changes
   - Screenshots (if UI changes)
   - Related issue numbers (if any)

## Publishing Updates

Version bumping is **automated** via GitHub Actions when you push to `main` with version tags in commit messages.

### Automated Version Bumping

When you push a commit to `main` with `[patch]`, `[minor]`, or `[major]` in the message:

```bash
git commit -m "Add icon preview feature [minor]"
git push origin main
```

The workflow automatically:

1. Reads current version from `extension.toml`
2. Increments version based on tag:
   - `[patch]` → 0.0.1 → 0.0.2 (bug fixes)
   - `[minor]` → 0.0.1 → 0.1.0 (new features)
   - `[major]` → 0.0.1 → 1.0.0 (breaking changes)
3. Updates `extension.toml` with new version
4. Updates `CHANGELOG.md` with entry
5. Creates git tag (e.g., `v0.1.0`)
6. Pushes tag and creates GitHub release

**Note:** Regular commits with `fix:`, `feat:`, etc. will NOT trigger version bumps. Only explicit `[patch]`, `[minor]`, or `[major]` tags will.

### Manual Version Bumping

If you prefer to bump versions manually:

1. **Update version** in `extension.toml`
2. **Update CHANGELOG.md**
3. **Build and test**: `cargo build --release`
4. **Commit changes**: `git commit -m "Bump version to X.X.X"`
5. **Tag release**: `git tag vX.X.X`
6. **Push**: `git push && git push --tags`

## Questions?

- **Issues**: [GitHub Issues](https://github.com/Michael-Obele/lucide-icons/issues)
- **Discussions**: Open an issue for questions
- **Lucide Icons**: [lucide.dev](https://lucide.dev)
- **Zed Extensions**: [Zed Extension Docs](https://zed.dev/docs/extensions)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
