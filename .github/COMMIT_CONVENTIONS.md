# Commit Message Guidelines

This project uses semantic versioning with automated version bumping via GitHub Actions.

## Version Bump Tags

Use these tags in your commit messages to trigger automatic version increments:

### `[patch]` — Bug Fixes & Small Changes

Increments: `0.0.1` → `0.0.2`

**Use for:**

- Bug fixes
- Documentation updates
- Small code improvements
- Typo corrections

**Examples:**

```bash
git commit -m "Fix icon search returning duplicates [patch]"
git commit -m "Update README installation instructions [patch]"
git commit -m "Fix typo in error message [patch]"
```

### `[minor]` — New Features

Increments: `0.0.1` → `0.1.0`

**Use for:**

- New features
- New slash commands
- Enhancements to existing features
- Backwards-compatible changes

**Examples:**

```bash
git commit -m "Add /lucide-browse command for icon categories [minor]"
git commit -m "Add tab completion for icon names [minor]"
git commit -m "Add SVG export functionality [minor]"
```

### `[major]` — Breaking Changes

Increments: `0.0.1` → `1.0.0`

**Use for:**

- Breaking API changes
- Incompatible updates
- Major rewrites
- Removal of features

**Examples:**

```bash
git commit -m "Rewrite extension API with new slash command structure [major]"
git commit -m "Remove deprecated /lucide-icon command [major]"
git commit -m "Change icon data format (breaking) [major]"
```

## Standard Commits (No Version Bump)

Regular commit prefixes do **NOT** trigger version bumps:

```bash
# These will NOT bump the version
git commit -m "fix: correct search algorithm"
git commit -m "feat: add new icon category"
git commit -m "chore: update dependencies"
git commit -m "docs: improve contributing guide"
git commit -m "refactor: simplify code structure"
git commit -m "test: add unit tests"
```

## Best Practices

### 1. One Tag Per Commit

❌ **Don't:** `git commit -m "Add feature [minor] and fix bug [patch]"`
✅ **Do:** Make separate commits for different types of changes

### 2. Tag Placement

Tags can be anywhere in the message:

```bash
git commit -m "[minor] Add new slash command"
git commit -m "Add new slash command [minor]"
git commit -m "Add [minor] new slash command"  # Works but not recommended
```

### 3. Clear Descriptions

```bash
# ❌ Vague
git commit -m "Updates [minor]"

# ✅ Clear
git commit -m "Add icon preview in Assistant [minor]"
```

### 4. Use Version Tags Sparingly

- Not every commit needs a version bump
- Bundle related changes before bumping
- Use `[patch]`, `[minor]`, `[major]` only for significant changes
- Regular `fix:` and `feat:` commits are fine without tags

### 5. Review Before Pushing to Main

Since version bumps are automatic on `main`:

- Test changes thoroughly
- Review commit messages
- Consider squashing commits before merge
- Use PRs to review before merging to `main`

## Workflow Behavior

When you push to `main` with a version tag:

1. ✅ Workflow detects `[patch]`, `[minor]`, or `[major]`
2. ✅ Calculates new version from current `extension.toml`
3. ✅ Updates `extension.toml` with new version
4. ✅ Adds entry to `CHANGELOG.md`
5. ✅ Commits as `github-actions[bot]`
6. ✅ Creates git tag (e.g., `v0.1.0`)
7. ✅ Creates GitHub release

## Examples by Scenario

### Scenario 1: Quick Bug Fix

```bash
# Current version: 0.1.5
git commit -m "Fix crash when searching empty string [patch]"
git push origin main
# New version: 0.1.6
```

### Scenario 2: New Feature

```bash
# Current version: 0.1.6
git commit -m "Add support for icon categories and filtering [minor]"
git push origin main
# New version: 0.2.0
```

### Scenario 3: Breaking Change

```bash
# Current version: 0.2.0
git commit -m "Rewrite slash command API (breaking change) [major]"
git push origin main
# New version: 1.0.0
```

### Scenario 4: Multiple Commits Without Versioning

```bash
# These don't trigger version bumps
git commit -m "refactor: clean up code"
git commit -m "docs: update examples"
git commit -m "chore: update dependencies"
git push origin main
# Version stays: 1.0.0
```

### Scenario 5: Bundled Changes

```bash
# Make multiple commits locally
git commit -m "Add feature A"
git commit -m "Add feature B"
git commit -m "Update docs"

# Squash before pushing
git rebase -i HEAD~3

# Add version tag to squashed commit
git commit --amend -m "Add features A and B with documentation [minor]"
git push origin main
# Version bumps once
```

## Troubleshooting

### Version didn't bump

- Check commit message has `[patch]`, `[minor]`, or `[major]`
- Verify you pushed to `main` branch
- Check GitHub Actions logs

### Wrong version bump

- Review commit message for correct tag
- Manually revert: update `extension.toml`, delete tag, push

### Want to skip workflow

- Don't include version tags in commit message
- Or add `[skip ci]` to commit message

## See Also

- [CONTRIBUTING.md](../CONTRIBUTING.md) — Full contribution guidelines
- [.github/workflows/README.md](workflows/README.md) — Workflow documentation
- [Semantic Versioning](https://semver.org/) — Version numbering specification
