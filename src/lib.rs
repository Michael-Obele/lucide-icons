use zed_extension_api::{self as zed, Result, SlashCommand, SlashCommandArgumentCompletion};

struct LucideIconsExtension {
    cached_icons: Option<Vec<String>>,
}

impl zed::Extension for LucideIconsExtension {
    fn new() -> Self {
        Self {
            cached_icons: None,
        }
    }

    fn slash_command_definitions(&mut self) -> Vec<SlashCommand> {
        vec![
            SlashCommand {
                name: "lucide-search".to_string(),
                description: "Search for Lucide icons by name".to_string(),
                tooltip_text: "Search Lucide icons".to_string(),
                requires_argument: true,
            },
            SlashCommand {
                name: "lucide-browse".to_string(),
                description: "Browse all available Lucide icons".to_string(),
                tooltip_text: "Open Lucide icons browser".to_string(),
                requires_argument: false,
            },
        ]
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
                    "home", "user", "search", "heart", "star", "settings", 
                    "edit", "trash", "download", "upload", "mail", "phone",
                    "camera", "image", "file", "folder", "lock", "unlock"
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
        &mut self,
        command: SlashCommand,
        args: Vec<String>,
    ) -> Result<String, String> {
        match command.name.as_str() {
            "lucide-search" => {
                let search_term = args.first().unwrap_or(&"".to_string()).clone();
                if search_term.is_empty() {
                    return Err("Please provide a search term".to_string());
                }

                // Generate HTML content for the icon search results
                let html_content = self.generate_icon_search_html(&search_term);
                Ok(format!(
                    "## Lucide Icons Search: {}\n\n{}\n\n[View on Lucide.dev](https://lucide.dev/icons/{})",
                    search_term,
                    html_content,
                    search_term
                ))
            }
            "lucide-browse" => {
                let html_content = self.generate_icon_browser_html();
                Ok(format!(
                    "## Lucide Icons Browser\n\n{}\n\n[Browse all icons on Lucide.dev](https://lucide.dev/icons/)",
                    html_content
                ))
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

        format!(
            "**Icon Found: {}**\n\n```html\n<lucide-{} size=\"24\" />\n```\n\n**Usage Examples:**\n- React: `<{} size={{24}} />`\n- Vue: `<lucide-{} :size=\"24\" />`\n- HTML: `<i data-lucide=\"{}\"></i>`",
            search_term,
            search_term,
            search_term.replace("-", "").to_uppercase(),
            search_term,
            search_term
        )
    }

    fn generate_icon_browser_html(&self) -> String {
        let popular_icons = vec![
            "home", "user", "settings", "search", "heart", "star", 
            "edit", "trash", "download", "upload", "mail", "phone"
        ];

        let mut html = "**Popular Icons:**\n\n".to_string();
        
        for icon in popular_icons {
            html.push_str(&format!(
                "- **{}**: `<lucide-{} />` [View](https://lucide.dev/icons/{})\n",
                icon, icon, icon
            ));
        }

        html.push_str("\n**How to use:**\n");
        html.push_str("1. Use `/lucide-search [icon-name]` to find specific icons\n");
        html.push_str("2. Copy the provided code snippets\n");
        html.push_str("3. Visit [lucide.dev](https://lucide.dev) for the complete library\n");

        html
    }
}

zed::register_extension!(LucideIconsExtension);