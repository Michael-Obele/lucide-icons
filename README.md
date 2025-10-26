# Lucide Icons for Zed

[![Zed Extension](https://img.shields.io/badge/zed-extension-blue)](https://zed.dev/extensions)
[![Version](https://img.shields.io/badge/version-0.0.1-green)](https://github.com/Michael-Obele/lucide-icons)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Lucide Icons for Zed brings the entire [Lucide icon](https://lucide.dev) library into the Zed Assistant. Search icons, browse curated lists, and grab framework-specific snippets without leaving your editor.

## ✨ Features

- 🔍 Slash commands for searching (`/lucide-search`) and browsing (`/lucide-browse`)
- 🧭 Autocomplete with keyword suggestions and icon metadata
- 📋 Code examples for React, Svelte 5, Vue, and HTML
- 🔗 Direct links to lucide.dev previews and CDN SVGs
- ⚙️ Quick customization tips for size, color, and stroke

## 📦 Installation

**Zed Extensions Gallery**

1. Open Zed and press `Cmd+Shift+X` (`Ctrl+Shift+X` on Linux/Windows).
2. Search for `Lucide Icons`.
3. Click **Install**.

**Manual (Development)**

1. Clone the repository: `git clone https://github.com/Michael-Obele/lucide-icons.git`.
2. Build the extension: `cargo build --release`.
3. In Zed, run `zed: install dev extension` and select the project folder.

## 🚀 Usage

- Open the Assistant (`Cmd+J` / `Ctrl+J`).
- Run `/lucide-search trash` to look up a specific icon with framework snippets.
- Run `/lucide-browse` to see curated icon categories and quick links.
- Use Tab completion while typing icon names to preview matching results.

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

## 🛠️ Development

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

Contributions are welcome:

1. Fork and branch (`git checkout -b feature/your-feature`).
2. Make and test your changes inside Zed.
3. Commit (`git commit -m "Describe change"`) and push.
4. Open a pull request with testing notes.

## 📄 License

MIT License — see `LICENSE` for details.

## 🙏 Acknowledgments

- [Lucide Icons](https://lucide.dev) for the icon set and documentation
- [Zed Editor](https://zed.dev) for the extension platform

## 📞 Support

- Issues & feature requests: [GitHub Issues](https://github.com/Michael-Obele/lucide-icons/issues)
- Lucide docs: [lucide.dev](https://lucide.dev)
- Zed docs: [zed.dev/docs](https://zed.dev/docs)
