# Lucide Icons Snippets

This folder contains code snippets for Lucide icons across different frameworks.

## Installation

These snippets are **optional**. The extension's slash commands (`/lucide-react`, `/lucide-svelte`, etc.) provide a faster, zero-setup workflow.

If you prefer traditional snippet-based autocomplete, follow the installation instructions in the main README.

## Snippet Files

- `javascript.json` - React snippets (JSX)
- `typescript.json` - React snippets with TypeScript support
- `svelte.json` - Svelte 5 snippets with runes
- `vue.json` - Vue 3 Composition API snippets
- `html.json` - Vanilla HTML/JavaScript snippets

## Usage After Installation

After copying snippets to `~/.config/zed/snippets/`, you can use these prefixes:

### React/TypeScript

- `luc` → Import statement
- `luci` → Icon component
- `lucic` → Icon with props (size, color, strokeWidth)
- `lucm` → Import + component
- `luct` → Import LucideIcon type (TypeScript only)

### Svelte

- `luc` → Import statement
- `luci` → Icon component
- `lucic` → Icon with props
- `lucm` → Import + component
- `luca` → Icon with reactive size ($state)

### Vue

- `luc` → Import statement
- `luci` → Icon component (Vue syntax)
- `lucic` → Icon with props
- `lucm` → Import + component
- `lucv` → Full component with Composition API

### HTML

- `luc` → Icon element
- `luci` → Icon with attributes
- `luccdn` → CDN script setup
- `lucsvg` → Complete icon + CDN setup

## Notes

- Snippets won't auto-update when the extension updates
- Restart Zed after copying snippets
- Use slash commands for icon-specific code generation
- Snippets provide templates; slash commands provide complete code
