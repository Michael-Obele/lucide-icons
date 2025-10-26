use zed_extension_api::{
    self as zed, Result, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection,
};

struct LucideIconsExtension {
    cached_icons: Option<Vec<String>>,
}

impl zed::Extension for LucideIconsExtension {
    fn new() -> Self {
        Self { cached_icons: None }
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "lucide-search" => {
                // In a real implementation, you'd fetch from Lucide API or cached list
                let common_icons = vec![
                    "home", "user", "search", "heart", "star", "settings", "edit", "trash",
                    "download", "upload", "mail", "phone", "camera", "image", "file", "folder",
                    "lock", "unlock",
                ];

                Ok(common_icons
                    .into_iter()
                    .map(|icon| SlashCommandArgumentCompletion {
                        label: format!("{} icon", icon),
                        new_text: icon.to_string(),
                        run_command: true,
                    })
                    .collect())
            }
            "lucide-browse" => Ok(vec![]),
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        match command.name.as_str() {
            "lucide-search" => {
                let search_term = args.first().unwrap_or(&"".to_string()).clone();
                if search_term.is_empty() {
                    return Err("Please provide a search term".to_string());
                }

                // Generate content for the icon search results
                let html_content = self.generate_icon_search_html(&search_term);
                let text = format!(
                    "## Lucide Icons Search: {}\n\n{}\n\n[View on Lucide.dev](https://lucide.dev/icons/{})",
                    search_term,
                    html_content,
                    search_term
                );

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: format!("Lucide Icon: {}", search_term),
                    }],
                    text,
                })
            }
            "lucide-browse" => {
                let html_content = self.generate_icon_browser_html();
                let text = format!(
                    "## Lucide Icons Browser\n\n{}\n\n[Browse all icons on Lucide.dev](https://lucide.dev/icons/)",
                    html_content
                );

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "Lucide Icons".to_string(),
                    }],
                    text,
                })
            }
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }
}

impl LucideIconsExtension {
    fn generate_icon_search_html(&self, search_term: &str) -> String {
        // In a real implementation, you would:
        // 1. Fetch from Lucide API or local cache
        // 2. Filter icons by search term
        // 3. Generate proper SVG or icon representations

        // Convert kebab-case to PascalCase for React/Svelte component names
        let pascal_case = search_term
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        format!(
            "**Icon Found: {}**\n\n**Usage Examples:**\n\n**React:**\n```jsx\nimport {{ {} }} from 'lucide-react';\n\n<{} size={{24}} />\n```\n\n**Svelte:**\n```svelte\n<script>\n  import {{ {} }} from '@lucide/svelte';\n</script>\n\n<{} size={{24}} />\n```\n\n**Vue:**\n```vue\n<template>\n  <{} :size=\"24\" />\n</template>\n\n<script setup>\nimport {{ {} }} from 'lucide-vue-next';\n</script>\n```\n\n**HTML:**\n```html\n<i data-lucide=\"{}\"></i>\n<script src=\"https://unpkg.com/lucide@latest\"></script>\n<script>\n  lucide.createIcons();\n</script>\n```",
            search_term,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            search_term
        )
    }

    fn generate_icon_browser_html(&self) -> String {
        let popular_icons = vec![
            ("home", "Home"),
            ("user", "User"),
            ("settings", "Settings"),
            ("search", "Search"),
            ("heart", "Heart"),
            ("star", "Star"),
            ("edit", "Edit"),
            ("trash", "Trash"),
            ("download", "Download"),
            ("upload", "Upload"),
            ("mail", "Mail"),
            ("phone", "Phone"),
        ];

        let mut html = "**Popular Icons:**\n\n".to_string();

        for (icon_name, _) in &popular_icons {
            html.push_str(&format!(
                "- **{}** — [View](https://lucide.dev/icons/{})\n",
                icon_name, icon_name
            ));
        }

        html.push_str("\n**Quick Usage:**\n\n");
        html.push_str("**React:** `import { Home } from 'lucide-react'; <Home size={24} />`\n\n");
        html.push_str(
            "**Svelte:** `import { Home } from '@lucide/svelte'; <Home size={24} />`\n\n",
        );
        html.push_str(
            "**Vue:** `import { Home } from 'lucide-vue-next'; <Home :size=\"24\" />`\n\n",
        );
        html.push_str("**HTML:** `<i data-lucide=\"home\"></i>`\n\n");

        html.push_str("\n**How to use:**\n");
        html.push_str(
            "1. Use `/lucide-search [icon-name]` to find specific icons with full code examples\n",
        );
        html.push_str("2. Copy the framework-specific code snippets\n");
        html.push_str(
            "3. Visit [lucide.dev](https://lucide.dev) for the complete library (1640+ icons)\n",
        );

        html
    }
}

zed::register_extension!(LucideIconsExtension);
