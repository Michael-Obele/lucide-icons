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

```
/lucide-search home
/lucide-search user
/lucide-search settings
```

The extension will provide:
- Icon preview information
- Usage examples for React, Vue, and HTML
- Direct link to the icon on lucide.dev

### Browse Popular Icons

Use the `/lucide-browse` slash command to see a list of popular icons:

```
/lucide-browse
```

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
```

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


