## lucide-icons — Quick instructions for AI coding agents

This repository is a small Rust-based Zed editor extension that exposes Lucide icon search/browse slash commands with multi-framework code snippet support (React, Vue, Svelte, HTML).
Keep guidance short and actionable — the maintainer expects edits to `src/lib.rs`, small helper modules, and Cargo manifest updates.

High-level overview

- Language: Rust (2021 edition). See `Cargo.toml` (crate-type = `cdylib`) — the crate is built as a native extension for Zed.
- Runtime: loaded by the Zed editor via the `zed_extension_api` crate (v0.6.0+). The extension registers slash commands via `extension.toml`.
- Key commands implemented: `/lucide-search` and `/lucide-browse` (defined in `extension.toml`, handlers in `src/lib.rs`).
- Multi-framework support: generates accurate code snippets for React, Vue, Svelte, and HTML based on official Lucide documentation.

Primary files to inspect

- `src/lib.rs` — core implementation and the best place to start. Slash command lifecycle handlers live here:
  - `complete_slash_command_argument()` — argument completions for icon names
  - `run_slash_command()` — produces `SlashCommandOutput` with markdown responses shown in Zed Assistant
  - helper methods: `generate_icon_search_html()` and `generate_icon_browser_html()`
- `extension.toml` — slash command definitions (Zed API v0.2.0+), metadata, and author information.
- `Cargo.toml` — build targets, dependencies (`zed_extension_api`, `serde`), and package metadata (license, authors, description).
- `README.md` — usage notes, installation flow, framework examples, and development workflow.

Common developer workflows (exact commands)

- Build release artifact: `cargo build --release` (creates the cdylib that Zed can load)
- Dev-testing: install as a dev extension inside Zed (see README: use the command palette `zed: install dev extension` and select the repo folder)
- Debugging: tail Zed logs via `zed: open log` in the editor and reproduce slash commands. There are no unit tests in repo — manual/integration testing inside Zed is expected.

Project-specific patterns and conventions

- Slash commands defined in `extension.toml`: commands are declared in TOML with `description` and `requires_argument` properties, then handlers implemented in `src/lib.rs` via `impl zed::Extension`.
- Multi-framework code generation: `generate_icon_search_html()` converts kebab-case icon names (e.g., `user-circle`) to PascalCase (e.g., `UserCircle`) for React/Svelte/Vue imports, while HTML uses the original kebab-case name.
- Framework-specific patterns follow official Lucide docs:
  - **React**: `import { IconName } from 'lucide-react'; <IconName size={24} />`
  - **Svelte**: `import { IconName } from '@lucide/svelte'; <IconName size={24} />`
  - **Vue**: `import { IconName } from 'lucide-vue-next'; <IconName :size="24" />`
  - **HTML**: `<i data-lucide="icon-name"></i>` with lucide.createIcons() call
- Response generation: UI output is generated as `SlashCommandOutput { sections: Vec<SlashCommandOutputSection>, text: String }`. Sections define ranges in the text that become collapsible in Zed Assistant. Prefer simple Markdown blocks with code fences for each framework.
- Data/cache: `LucideIconsExtension` keeps an optional `cached_icons: Option<Vec<String>>`. If adding icon data, populate this cache lazily and use it for completions + search filtering.
- No async/await: current code is synchronous. If you introduce network calls (Lucide API), keep them off the main thread or use a background worker pattern; document any new threading/async assumptions in the code.

Integration points & extension API

- `zed_extension_api` v0.6.0+ is the entry point. The repo uses `zed::register_extension!(LucideIconsExtension);` to expose the extension.
- Slash commands registered in `extension.toml` under `[slash_commands.command-name]` section with properties: `description` (string), `requires_argument` (bool).
- Slash command completions use `SlashCommandArgumentCompletion { label, new_text, run_command }` — set `run_command` to true when you want the selected completion to immediately execute.
- Return type: `run_slash_command()` returns `Result<SlashCommandOutput, String>` where `SlashCommandOutput` contains `sections: Vec<SlashCommandOutputSection>` and `text: String`.

Where to change behavior (examples)

- To add icon data from a local JSON file: add a small module (e.g. `src/icons.rs`) that loads `lucide-icons/*.json` at build time or lazily at runtime, then set `self.cached_icons = Some(...);` in the extension before serving completions.
- To implement fuzzy search: change `complete_slash_command_argument()` to filter `cached_icons` using the chosen algorithm and return mapped `SlashCommandArgumentCompletion`s.
- To add a new framework (e.g., Angular, Solid): update `generate_icon_search_html()` to include a new code fence block with framework-specific import/usage pattern. Reference official Lucide docs for that framework.
- To add icon categories: extend `generate_icon_browser_html()` to group icons by category (actions, navigation, etc.) with separate sections.

Edge cases & guardrails

- Validate arguments in `run_slash_command()` (current code errors when search term is empty). Keep errors as `Err(String)` with human-readable messages.
- Avoid large payloads in completion lists — keep completions to a reasonable size (20–50 items); prefer paging or a small 'more results' pattern.
- PascalCase conversion: ensure the icon name transformation handles edge cases (multiple hyphens, numbers, etc.) correctly when generating component names.

Quick checklist for PRs from an AI agent

- Small focused change: update `src/lib.rs` or add a small module under `src/`.
- Update `Cargo.toml` only if adding dependencies — prefer minimal deps and pin versions consistent with repo.
- Update `extension.toml` if adding new slash commands or changing metadata.
- Verify build locally: `cargo build --release`.
- Test multi-framework snippets: ensure React/Vue/Svelte/HTML examples match official Lucide docs.
- Explain how to test in the PR description (how to install dev extension in Zed and which slash command to run).

If anything is unclear

- Ask the maintainer whether new network calls should be added to the extension or implemented as an external service; note the repo currently expects Zed-local behavior.
- Consult official Lucide docs at https://lucide.dev for framework-specific API changes before updating code snippets.

End — please review and tell me any missing details you want included (e.g., how to publish the extension to Zed marketplace, CI steps, or where to put generated icon assets).
