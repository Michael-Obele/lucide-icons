# Lucide Icons for Zed

[![Zed Extension](https://img.shields.io/badge/zed-extension-blue)](https://zed.dev/extensions)
[![Version](https://img.shields.io/github/v/tag/Michael-Obele/lucide-icons)](https://github.com/Michael-Obele/lucide-icons/tags)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Lucide Icons for Zed brings the entire [Lucide icon](https://lucide.dev) library into the Zed Assistant. Search icons, browse curated lists, and grab framework-specific snippets without leaving your editor.

<!-- Demo video placeholder - uncomment when available
## 🎥 Demo

[Demo video or GIF showing the extension in action]
-->

## ✨ Features

- 🔍 **Multiple slash commands** for quick access:
  - `/lucide-search` — Search all icons with multi-framework examples
  - `/lucide-browse` — Browse curated icon categories
  - `/lucide-react` — Get React-specific code for any icon
  - `/lucide-svelte` — Get Svelte 5-specific code for any icon
  - `/lucide-vue` — Get Vue 3-specific code for any icon
  - `/lucide-html` — Get vanilla HTML/JavaScript code for any icon
- 🧭 **Smart autocomplete** with keyword suggestions and icon metadata
- 📋 **Framework-focused output** — only see the code you need
- 🎨 **Raw SVG support** — copy-paste ready SVG code with CDN links
- 🔗 **Direct links** to lucide.dev previews and CDN resources
- ⚙️ **Quick customization** tips for size, color, and stroke

## 📦 Installation

> **⚠️ Note: Not Available on Marketplace**
>
> This extension is **not available on the Zed Extensions Gallery**. As noted in [PR #3670](https://github.com/zed-industries/extensions/pull/3670), slash commands are intended for providing context to AI agents, not for general utility features like icon insertion. This extension will be reconsidered if/when Zed's extension API expands to support custom palette actions, keybindings, and direct buffer edits.

**Manual Installation (Required)**

1. Clone the repository: `git clone https://github.com/Michael-Obele/lucide-icons.git`.
2. Build the extension: `cargo build --release`.
3. In Zed, run `zed: install dev extension` (via command palette) and select the project folder.

## 🚀 Usage

> **📌 Important: Text Threads Only**
>
> These slash commands only work in **text threads** within Zed's Agent Panel. Text threads are an editor-based interface for conversing with language models, where you can use slash commands to enhance your AI interactions.
>
> **To use Lucide icon commands:**
>
> 1. Open the Agent Panel with `Cmd+J` (`Ctrl+J` on Linux/Windows)
> 2. Start a new text thread or open an existing one
> 3. Type `/` at the beginning of a line to see all available slash commands
> 4. Select any Lucide command (e.g., `/lucide-search`, `/lucide-react`) from the list
>
> **What are text threads?** Text threads in the Agent Panel work like a regular editor with message blocks for "You", "Assistant", and "System" roles. You can edit messages, use multiple cursors, and insert slash commands to add dynamic content like icon code snippets.
>
> 📖 Learn more: [Text Threads Documentation](https://zed.dev/docs/ai/text-threads.html)

### Slash Commands

- **`/lucide-search <icon-name>`** — Search for an icon and get code examples for all frameworks
- **`/lucide-browse`** — Browse popular icons organized by category
- **`/lucide-react <icon-name>`** — React-only code with hooks, styling, and interaction examples
- **`/lucide-svelte <icon-name>`** — Svelte 5-only code with runes ($state, $derived) examples
- **`/lucide-vue <icon-name>`** — Vue 3-only code with Composition API examples
- **`/lucide-html <icon-name>`** — Vanilla HTML/JavaScript with CDN and raw SVG options

**Tab Completion:** All commands support tab completion — start typing an icon name to see suggestions with related tags.

### Command Examples

```bash
# Get all framework examples for a home icon
/lucide-search home

# Get only React code for a trash icon
/lucide-react trash

# Get only Svelte code for a user icon
/lucide-svelte user

# Browse all available icons by category
/lucide-browse
```

### Framework Snippets

```jsx
// React
import { Home } from "lucide-react";

export function Header() {
  return <Home size={24} strokeWidth={1.5} />;
}
```

```html
<script>
  import { Home } from "@lucide/svelte";
  let iconSize = $state(24);
</script>

<Home size="{iconSize}" />
```

```html
<template>
  <Home :size="24" :stroke-width="1.5" />
</template>

<script setup>
  import { Home } from "lucide-vue-next";
</script>
```

```html
<i data-lucide="home" data-lucide-size="24" data-lucide-stroke-width="1.5"></i>
<script src="https://unpkg.com/lucide@latest"></script>
<script>
  lucide.createIcons();
</script>
```

## 📝 Optional: Code Snippets Installation

While slash commands provide the fastest workflow, you can optionally install code snippets for traditional autocomplete-style usage (e.g., type `luc` → get completions).

**Note:** This requires manual installation and won't auto-update with the extension. We recommend using slash commands for the best experience.

### ⚠️ Important: Check for Existing Snippets First

**The commands below will overwrite any existing snippet files** (e.g., `javascript.json`, `typescript.json`, etc.).

**Before installing, check if you have existing snippets:**

```bash
ls ~/.config/zed/snippets/
```

If you see files like `javascript.json`, `typescript.json`, etc., you have existing snippets that will be replaced.

### Installation Options

#### Option A: Fresh Install (No Existing Snippets)

**macOS:**

```bash
cp -r ~/Library/Application\ Support/Zed/extensions/installed/lucide-icons/snippets/* ~/.config/zed/snippets/
```

**Linux:**

```bash
cp -r ~/.local/share/zed/extensions/installed/lucide-icons/snippets/* ~/.config/zed/snippets/
```

#### Option B: Backup First (If You Have Existing Snippets)

**macOS:**

```bash
# Backup existing snippets
cp -r ~/.config/zed/snippets ~/.config/zed/snippets.backup

# Install Lucide snippets
cp -r ~/Library/Application\ Support/Zed/extensions/installed/lucide-icons/snippets/* ~/.config/zed/snippets/
```

**Linux:**

```bash
# Backup existing snippets
cp -r ~/.config/zed/snippets ~/.config/zed/snippets.backup

# Install Lucide snippets
cp -r ~/.local/share/zed/extensions/installed/lucide-icons/snippets/* ~/.config/zed/snippets/
```

#### Option C: Manual Merge (Safest)

1. Open the extension snippets folder and your snippets folder side-by-side
2. View our snippet files on [GitHub](https://github.com/Michael-Obele/lucide-icons/tree/main/snippets)
3. Manually copy the snippets you want into your existing files
4. This way you keep your existing snippets and add only the Lucide ones you need

**After installation**, restart Zed or reload the window for snippets to take effect.

### Available Snippet Prefixes

Once installed, you can use these prefixes in your code:

- **React/TypeScript:** `luc`, `luci`, `lucic`, `lucm`, `luct` (TypeScript only)
- **Svelte:** `luc`, `luci`, `lucic`, `lucm`, `luca`
- **Vue:** `luc`, `luci`, `lucic`, `lucm`, `lucv`
- **HTML:** `luc`, `luci`, `luccdn`, `lucsvg`

## �🛠️ Development

- **Prerequisites:** Rust (via [rustup](https://rustup.rs/)), Cargo, and the `wasm32-wasip1` target (`rustup target add wasm32-wasip1`).
- **Build:** `cargo build --release`
- **Artifacts:** `target/wasm32-wasip1/release/lucide_icons.wasm`
- **Test:** Install as a dev extension, open the Assistant, and run the slash commands.
- **Debug:** Use `zed: open log` inside Zed or launch Zed with `zed --foreground` for verbose output.

## 🗺️ Roadmap

- Add fuzzy search scoring and ranking improvements
- Provide icon categories with richer metadata
- Offer ready-to-copy snippets for more frameworks (Solid, Angular, Web Components)
- Explore inline preview once Zed exposes rendering APIs

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on:

- Development setup and prerequisites
- Project structure and code conventions
- Testing procedures
- Pull request process

**Automated Versioning:** Commits to `main` with `[patch]`, `[minor]`, or `[major]` tags automatically bump the version, update the changelog, and create releases.

## 📄 License

MIT License — see `LICENSE` for details.

## 🙏 Acknowledgments

- [Lucide Icons](https://lucide.dev) for the icon set and documentation
- [Zed Editor](https://zed.dev) for the extension platform

## 📞 Support

- Issues & feature requests: [GitHub Issues](https://github.com/Michael-Obele/lucide-icons/issues)
- Lucide docs: [lucide.dev](https://lucide.dev)
- Zed docs: [zed.dev/docs](https://zed.dev/docs)
