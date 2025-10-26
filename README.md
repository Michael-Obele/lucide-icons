# Lucide Icons for Zed

A Zed extension that allows you to search and browse Lucide Icons directly within the editor using slash commands.

![Zed Extension](https://img.shields.io/badge/zed-extension-blue)
![Version](https://img.shields.io/badge/version-0.0.1-green)

## Features

- 🔍 **Search Icons**: Find specific Lucide icons by name with autocompletion
- 📚 **Browse Icons**: View popular icons and usage examples
- 📋 **Copy Code**: Get ready-to-use code snippets for React, Vue, and HTML
- 🔗 **Quick Links**: Direct links to official Lucide documentation

## Installation

### From Zed Extensions Gallery

1. Open Zed
2. Press `Cmd+Shift+X` (or `Ctrl+Shift+X` on Linux/Windows)
3. Search for "Lucide Icons"
4. Click Install

### Manual Installation (Development)

1. Clone this repository
2. In Zed, open the command palette and run `zed: install dev extension`
3. Select the extension directory

## Usage

### Search for Icons

Use the `/lucide-search` slash command to find specific icons:

````
# Lucide Icons for Zed

[![Zed Extension](https://img.shields.io/badge/zed-extension-blue)](https://zed.dev/extensions)
[![Version](https://img.shields.io/badge/version-0.0.1-green)](https://github.com/Michael-Obele/lucide-icons)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Zed extension that allows you to search and browse [Lucide Icons](https://lucide.dev) (1640+ icons) directly within the editor using slash commands. Get instant access to ready-to-use code snippets for React, Vue, Svelte, and HTML.

## ✨ Features

- 🔍 **Search Icons** — Find specific Lucide icons by name with autocompletion
- 📚 **Browse Popular Icons** — View commonly used icons and quick examples
- 📋 **Multi-Framework Support** — Get code snippets for React, Vue, Svelte, and HTML
- ⚡ **Fast & Lightweight** — Instant access without leaving your editor
- 🔗 **Direct Links** — Quick navigation to official Lucide documentation

## 📦 Installation

### From Zed Extensions Gallery
1. Open Zed
2. Press `Cmd+Shift+X` (or `Ctrl+Shift+X` on Linux/Windows)
3. Search for "Lucide Icons"
4. Click Install

### Manual Installation (Development)
1. Clone this repository:
   ```bash
   git clone https://github.com/Michael-Obele/lucide-icons.git
````

2. In Zed, open the command palette and run `zed: install dev extension`
3. Select the extension directory

## 🚀 Usage

### Search for Icons

Use the `/lucide-search` slash command in the Assistant to find specific icons:

```
/lucide-search home
/lucide-search user
/lucide-search settings
```

The extension provides:

- Icon usage information
- Framework-specific code examples (React, Vue, Svelte, HTML)
- Direct link to the icon on lucide.dev

### Browse Popular Icons

Use the `/lucide-browse` slash command to see a curated list of popular icons:

```
/lucide-browse
```

This displays:

- Most commonly used icons with links
- Quick-reference usage examples
- Framework-agnostic syntax guide

## 📝 Supported Frameworks

The extension provides accurate code snippets for multiple frameworks based on [official Lucide documentation](https://lucide.dev):

### React

```jsx
import { Home } from "lucide-react";

<Home size={24} />;
```

### Svelte

```svelte
<script>
  import { Home } from '@lucide/svelte';
</script>

<Home size={24} />
```

### Vue

```vue
<template>
  <Home :size="24" />
</template>

<script setup>
import { Home } from "lucide-vue-next";
</script>
```

### HTML

```html
<i data-lucide="home"></i>
<script src="https://unpkg.com/lucide@latest"></script>
<script>
  lucide.createIcons();
</script>
```

## 🛠️ Development

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))
- Cargo
- `wasm32-wasip1` target: `rustup target add wasm32-wasip1`

### Building

```bash
cargo build --release
```

The compiled extension will be in `target/wasm32-wasip1/release/lucide_icons.wasm`

### Testing

1. Install as dev extension in Zed (see Manual Installation above)
2. Use the slash commands (`/lucide-search`, `/lucide-browse`) to test functionality
3. Check Zed logs (`zed: open log`) for any errors or debugging output

## 🗺️ Roadmap

- [ ] Add icon preview images in search results
- [ ] Implement fuzzy search for better icon discovery
- [ ] Add support for icon categories and filtering
- [ ] Create visual icon browser with webview
- [ ] Support for icon variants (size, stroke width, color options)
- [ ] Add custom icon sets support
- [ ] Integrate with clipboard for quick copying

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test the extension in Zed
5. Commit your changes (`git commit -m 'Add some amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Lucide Icons](https://lucide.dev) — Beautiful & consistent icon toolkit made by the community
- [Zed Editor](https://zed.dev) — High-performance, multiplayer code editor
- Inspired by VS Code's Lucide Icons extensions

---

**Note**: This extension uses Zed's slash command system to provide seamless icon search and browsing within the Assistant panel.

```

The extension will provide:
- Icon preview information
- Usage examples for React, Vue, and HTML
- Direct link to the icon on lucide.dev

### Browse Popular Icons

Use the `/lucide-browse` slash command to see a list of popular icons:

```

/lucide-browse

````

This shows:
- Most commonly used icons
- Quick copy-paste code snippets
- Links to the full Lucide library

## Supported Frameworks

The extension provides code snippets for:

- **React**: `<Home size={24} />`
- **Vue**: `<lucide-home :size="24" />`
- **HTML**: `<i data-lucide="home"></i>`
- **Web Components**: `<lucide-home size="24" />`

## Development

### Prerequisites
- Rust (installed via rustup)
- Cargo

### Building

```bash
cargo build --release
````

### Testing

1. Install as dev extension in Zed
2. Use the slash commands to test functionality
3. Check Zed logs (`zed: open log`) for any errors

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test the extension
5. Submit a pull request

## Roadmap

- [ ] Add icon preview images
- [ ] Implement icon search with fuzzy matching
- [ ] Add support for custom icon sets
- [ ] Create webview for visual icon browser
- [ ] Add icon category filtering
- [ ] Support for icon variants (size, stroke, fill)

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Lucide Icons](https://lucide.dev) - Beautiful & consistent icon toolkit
- [Zed Editor](https://zed.dev) - High-performance code editor

---

**Note**: This extension is inspired by VS Code's Lucide Icons extensions and adapts the functionality for Zed's unique slash command system.
