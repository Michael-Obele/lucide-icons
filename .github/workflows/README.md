# GitHub Workflows

This directory contains automated workflows for the Lucide Icons extension.

## Version Bump Workflow

**File:** `version-bump.yml`

### Purpose

Automatically increments the extension version when commits contain specific version tags.

### Trigger

Runs on push to `main` branch when commit message contains:

- `[major]` — Breaking changes (0.0.1 → 1.0.0)
- `[minor]` — New features (0.0.1 → 0.1.0)
- `[patch]` — Bug fixes (0.0.1 → 0.0.2)

### What It Does

1. ✅ Parses commit message for version tag
2. ✅ Extracts current version from `extension.toml`
3. ✅ Calculates new version based on semver rules
4. ✅ Updates `extension.toml` with new version
5. ✅ Updates `CHANGELOG.md` with new entry
6. ✅ Commits changes as `github-actions[bot]`
7. ✅ Creates git tag (e.g., `v0.1.0`)
8. ✅ Pushes tag and commit
9. ✅ Creates GitHub release

### Usage Examples

```bash
# Patch bump (0.0.1 → 0.0.2)
git commit -m "Fix icon search bug [patch]"

# Minor bump (0.0.1 → 0.1.0)
git commit -m "Add new browse command [minor]"

# Major bump (0.0.1 → 1.0.0)
git commit -m "Rewrite extension API [major]"

# No version bump (ignored by workflow)
git commit -m "fix: typo in README"
git commit -m "feat: add icon categories"
```

### Important Notes

- **Only explicit tags trigger bumps:** `fix:`, `feat:`, `chore:`, etc. will NOT trigger version bumps
- **One tag per commit:** Use only one of `[major]`, `[minor]`, or `[patch]` per commit
- **Tag placement:** Tag can be anywhere in the commit message
- **Requires permissions:** Workflow needs write access to repository

### Workflow Permissions

The workflow requires:

- `contents: write` — To push commits and tags
- `GITHUB_TOKEN` — Automatically provided by GitHub Actions

### Manual Override

If you need to bump version without the workflow:

```bash
# Edit extension.toml and CHANGELOG.md manually
vim extension.toml
vim CHANGELOG.md

# Commit and tag
git commit -m "Bump version to 0.2.0"
git tag v0.2.0
git push && git push --tags
```

### Troubleshooting

**Workflow doesn't run:**

- Ensure commit message contains `[major]`, `[minor]`, or `[patch]`
- Check workflow logs in Actions tab
- Verify `main` is the default branch

**Permission denied error:**

- Check repository settings → Actions → General
- Enable "Read and write permissions" for workflows

**Version not updated:**

- Check `extension.toml` format: `version = "X.X.X"`
- Verify version follows semver (major.minor.patch)
