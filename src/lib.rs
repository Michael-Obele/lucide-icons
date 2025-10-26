use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;
use zed_extension_api::{
    self as zed, http_client, serde_json, Result, SlashCommand, SlashCommandArgumentCompletion,
    SlashCommandOutput, SlashCommandOutputSection,
};

struct LucideIconsExtension {
    cached_icons: RwLock<Option<HashMap<String, Vec<String>>>>,
}

impl zed::Extension for LucideIconsExtension {
    fn new() -> Self {
        Self {
            cached_icons: RwLock::new(None),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "lucide-search" => {
                // Lazy-load icon data if not cached
                if self.cached_icons.read().unwrap().is_none() {
                    self.fetch_icon_data();
                }

                let search_term = args.first().map(|s| s.to_lowercase()).unwrap_or_default();

                let icons = self
                    .cached_icons
                    .read()
                    .unwrap()
                    .as_ref()
                    .map(|map| {
                        map.keys()
                            .filter(|name| {
                                if search_term.is_empty() {
                                    // Return top 20 popular icons by default
                                    [
                                        "home", "user", "search", "heart", "star", "settings",
                                        "edit", "trash", "download", "upload", "mail", "phone",
                                        "camera", "image", "file", "folder", "lock", "unlock",
                                        "check", "x",
                                    ]
                                    .contains(&name.as_str())
                                } else {
                                    // Fuzzy match on icon name or tags
                                    name.contains(&search_term)
                                        || map
                                            .get(*name)
                                            .map(|tags| {
                                                tags.iter().any(|tag| tag.contains(&search_term))
                                            })
                                            .unwrap_or(false)
                                }
                            })
                            .take(50) // Limit to 50 results
                            .map(|name| {
                                let tags = map.get(name).map(|t| t.join(", ")).unwrap_or_default();
                                let label = if tags.is_empty() {
                                    name.to_string()
                                } else {
                                    format!("{} ({})", name, &tags[..tags.len().min(50)])
                                };

                                SlashCommandArgumentCompletion {
                                    label,
                                    new_text: name.to_string(),
                                    run_command: true,
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new);

                Ok(icons)
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
    ) -> Result<SlashCommandOutput, String> {
        // Ensure icons are loaded
        if self.cached_icons.read().unwrap().is_none() {
            self.fetch_icon_data();
        }

        match command.name.as_str() {
            "lucide-search" => {
                let search_term = args.first().ok_or("Please provide an icon name")?.clone();

                // Check if icon exists
                let icon_exists = self
                    .cached_icons
                    .read()
                    .unwrap()
                    .as_ref()
                    .map(|map| map.contains_key(&search_term))
                    .unwrap_or(false);

                if !icon_exists {
                    return Err(format!(
                        "Icon '{}' not found. Use tab completion to see available icons.",
                        search_term
                    ));
                }

                let tags = self
                    .cached_icons
                    .read()
                    .unwrap()
                    .as_ref()
                    .and_then(|map| map.get(&search_term))
                    .map(|t| t.join(", "))
                    .unwrap_or_default();

                let html_content = self.generate_icon_search_html(&search_term, &tags);
                let text = format!(
                    "## Lucide Icon: {}\n\n{}\n\n---\n\n[View on Lucide.dev](https://lucide.dev/icons/{})",
                    search_term, html_content, search_term
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
                let icon_count = self
                    .cached_icons
                    .read()
                    .unwrap()
                    .as_ref()
                    .map(|map| map.len())
                    .unwrap_or(0);

                let html_content = self.generate_icon_browser_html();
                let text = format!(
                    "## Lucide Icons Browser\n\n**Total Icons: {}**\n\n{}\n\n---\n\n[Browse all icons on Lucide.dev](https://lucide.dev/icons/)",
                    icon_count, html_content
                );

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "Lucide Icons Browser".to_string(),
                    }],
                    text,
                })
            }
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }
}

impl LucideIconsExtension {
    /// Fetch icon data from Lucide CDN and cache it
    fn fetch_icon_data(&self) {
        // Build HTTP request
        let request = http_client::HttpRequest {
            url: "https://cdn.jsdelivr.net/npm/lucide-static@latest/tags.json".to_string(),
            method: http_client::HttpMethod::Get,
            headers: vec![],
            body: None,
            redirect_policy: http_client::RedirectPolicy::FollowAll,
        };

        // Try to fetch from CDN
        match http_client::fetch(&request) {
            Ok(response) => {
                if let Ok(json) = serde_json::from_slice::<Value>(&response.body) {
                    if let Some(obj) = json.as_object() {
                        let mut icons = HashMap::new();
                        for (key, value) in obj {
                            if let Some(tags) = value.as_array() {
                                let tag_strings: Vec<String> = tags
                                    .iter()
                                    .filter_map(|v| v.as_str())
                                    .map(String::from)
                                    .collect();
                                icons.insert(key.clone(), tag_strings);
                            }
                        }
                        *self.cached_icons.write().unwrap() = Some(icons);
                        return;
                    }
                }
            }
            Err(_) => {
                // Fetch failed, use fallback icons
            }
        }

        // Fallback to a basic set of popular icons
        *self.cached_icons.write().unwrap() = Some(self.fallback_icons());
    }

    /// Fallback icon list in case CDN fetch fails
    fn fallback_icons(&self) -> HashMap<String, Vec<String>> {
        let mut icons = HashMap::new();
        let popular = vec![
            ("home", vec!["house", "building"]),
            ("user", vec!["person", "account", "profile"]),
            ("search", vec!["find", "magnifier", "lens"]),
            ("heart", vec!["like", "love", "favorite"]),
            ("star", vec!["favorite", "rating", "bookmark"]),
            ("settings", vec!["preferences", "config", "gear"]),
            ("edit", vec!["pencil", "write", "modify"]),
            ("trash", vec!["delete", "remove", "bin"]),
            ("download", vec!["save", "import"]),
            ("upload", vec!["export", "send"]),
            ("mail", vec!["email", "message", "envelope"]),
            ("phone", vec!["call", "telephone"]),
            ("camera", vec!["photo", "picture"]),
            ("image", vec!["picture", "photo"]),
            ("file", vec!["document", "page"]),
            ("folder", vec!["directory", "files"]),
            ("lock", vec!["secure", "private"]),
            ("unlock", vec!["open", "public"]),
            ("check", vec!["tick", "done", "complete"]),
            ("x", vec!["close", "cancel", "remove"]),
        ];

        for (name, tags) in popular {
            icons.insert(
                name.to_string(),
                tags.into_iter().map(String::from).collect(),
            );
        }
        icons
    }

    /// Convert kebab-case to PascalCase for React/Vue/Svelte components
    fn to_pascal_case(&self, kebab: &str) -> String {
        kebab
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    }

    /// Get emoji representation for common icon categories
    fn get_icon_emoji(&self, icon_name: &str, tags: &[String]) -> &str {
        // Check icon name and tags for category matches
        let name_and_tags = format!("{} {}", icon_name, tags.join(" ")).to_lowercase();

        if name_and_tags.contains("home") || name_and_tags.contains("house") {
            "🏠"
        } else if name_and_tags.contains("user")
            || name_and_tags.contains("person")
            || name_and_tags.contains("profile")
        {
            "👤"
        } else if name_and_tags.contains("heart")
            || name_and_tags.contains("like")
            || name_and_tags.contains("love")
        {
            "❤️"
        } else if name_and_tags.contains("star") || name_and_tags.contains("favorite") {
            "⭐"
        } else if name_and_tags.contains("search") || name_and_tags.contains("find") {
            "🔍"
        } else if name_and_tags.contains("mail")
            || name_and_tags.contains("email")
            || name_and_tags.contains("message")
        {
            "📧"
        } else if name_and_tags.contains("phone") || name_and_tags.contains("call") {
            "📞"
        } else if name_and_tags.contains("camera") || name_and_tags.contains("photo") {
            "📷"
        } else if name_and_tags.contains("calendar") {
            "📅"
        } else if name_and_tags.contains("clock") || name_and_tags.contains("time") {
            "⏰"
        } else if name_and_tags.contains("settings") || name_and_tags.contains("gear") {
            "⚙️"
        } else if name_and_tags.contains("trash") || name_and_tags.contains("delete") {
            "🗑️"
        } else if name_and_tags.contains("lock") || name_and_tags.contains("secure") {
            "🔒"
        } else if name_and_tags.contains("unlock") {
            "🔓"
        } else if name_and_tags.contains("check")
            || name_and_tags.contains("tick")
            || name_and_tags.contains("done")
        {
            "✅"
        } else if name_and_tags.contains("alert") || name_and_tags.contains("warning") {
            "⚠️"
        } else if name_and_tags.contains("info") || name_and_tags.contains("information") {
            "ℹ️"
        } else if name_and_tags.contains("file") || name_and_tags.contains("document") {
            "📄"
        } else if name_and_tags.contains("folder") {
            "📁"
        } else if name_and_tags.contains("download") {
            "⬇️"
        } else if name_and_tags.contains("upload") {
            "⬆️"
        } else if name_and_tags.contains("cloud") {
            "☁️"
        } else if name_and_tags.contains("bell") || name_and_tags.contains("notification") {
            "🔔"
        } else if name_and_tags.contains("chart") || name_and_tags.contains("graph") {
            "📊"
        } else if name_and_tags.contains("image") || name_and_tags.contains("picture") {
            "🖼️"
        } else {
            "🎨" // Default icon emoji
        }
    }

    /// Generate HTML content for icon search results with code snippets
    fn generate_icon_search_html(&self, icon_name: &str, tags: &str) -> String {
        let pascal_case = self.to_pascal_case(icon_name);
        let tags_vec: Vec<String> = tags.split(", ").map(|s| s.to_string()).collect();
        let emoji = self.get_icon_emoji(icon_name, &tags_vec);

        let tags_section = if !tags.is_empty() {
            format!("**Related Terms:** {}\n\n", tags)
        } else {
            String::new()
        };

        format!(
            "{} **{}**\n\n\
{}\
---\n\n\
### 📋 Quick Links\n\n\
- [🌐 View on Lucide.dev](https://lucide.dev/icons/{}) — Interactive preview with all variants\n\
- [📦 CDN SVG](https://cdn.jsdelivr.net/npm/lucide-static@latest/icons/{}.svg) — Direct SVG file\n\
- [📚 Documentation](https://lucide.dev/guide/packages/lucide-react) — Framework guides\n\n\
---\n\n\
### 💻 Code Examples\n\n\
#### React\n\
```jsx\n\
import {{ {} }} from 'lucide-react';\n\n\
function MyComponent() {{\n\
  return (\n\
    <>\n\
      {{/* Basic usage */}}\n\
      <{} size={{24}} />\n\
      \n\
      {{/* With custom color and stroke */}}\n\
      <{} size={{24}} color=\"#ff0000\" strokeWidth={{2}} />\n\
      \n\
      {{/* As a clickable icon */}}\n\
      <{} size={{24}} onClick={{() => console.log('clicked')}} />\n\
    </>\n\
  );\n\
}}\n\
```\n\n\
#### Svelte 5\n\
```svelte\n\
<script>\n\
  import {{ {} }} from 'lucide-svelte';\n\
</script>\n\n\
<!-- Basic usage -->\n\
<{} size={{24}} />\n\n\
<!-- With custom styling -->\n\
<{} size={{24}} color=\"#ff0000\" strokeWidth={{2}} />\n\n\
<!-- With click handler -->\n\
<{} size={{24}} onclick={{() => console.log('clicked')}} />\n\
```\n\n\
#### Vue 3\n\
```vue\n\
<template>\n\
  <div>\n\
    <!-- Basic usage -->\n\
    <{} :size=\"24\" />\n\
    \n\
    <!-- With custom styling -->\n\
    <{} :size=\"24\" color=\"#ff0000\" :stroke-width=\"2\" />\n\
    \n\
    <!-- With click handler -->\n\
    <{} :size=\"24\" @click=\"handleClick\" />\n\
  </div>\n\
</template>\n\n\
<script setup>\n\
import {{ {} }} from 'lucide-vue-next';\n\n\
const handleClick = () => {{\n\
  console.log('clicked');\n\
}};\n\
</script>\n\
```\n\n\
#### HTML + JavaScript\n\
```html\n\
<!-- Basic usage -->\n\
<i data-lucide=\"{}\"></i>\n\n\
<!-- With custom attributes -->\n\
<i data-lucide=\"{}\" data-lucide-size=\"24\" data-lucide-color=\"red\"></i>\n\n\
<!-- Load Lucide -->\n\
<script src=\"https://unpkg.com/lucide@latest\"></script>\n\
<script>\n\
  // Initialize all icons\n\
  lucide.createIcons();\n\
</script>\n\
```\n\n\
---\n\n\
### 🎨 Customization Options\n\n\
All Lucide icons support these props:\n\
- `size` — Icon size (number or string, default: 24)\n\
- `color` — Icon color (CSS color value, default: currentColor)\n\
- `strokeWidth` — Stroke width (number, default: 2)\n\
- `absoluteStrokeWidth` — Override automatic stroke width scaling\n\
- Standard HTML/SVG attributes (class, style, etc.)\n\n\
### 📦 Installation\n\n\
```bash\n\
# React\n\
npm install lucide-react\n\n\
# Svelte\n\
npm install lucide-svelte\n\n\
# Vue\n\
npm install lucide-vue-next\n\
```",
            emoji,
            icon_name,
            tags_section,
            icon_name,
            icon_name,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            pascal_case,
            icon_name,
            icon_name
        )
    }

    /// Generate HTML content for the icon browser with categories
    fn generate_icon_browser_html(&self) -> String {
        let mut html = String::from("# Lucide Icons Browser\n\n");
        html.push_str("Browse popular icons organized by category. Click any icon name to view it on lucide.dev.\n\n");
        html.push_str("---\n\n");

        // Navigation icons
        html.push_str("## 🧭 Navigation & UI\n\n");
        let nav_icons = vec![
            ("home", "🏠", "Home page, dashboard"),
            ("menu", "☰", "Navigation menu, hamburger"),
            ("search", "🔍", "Search functionality"),
            ("settings", "⚙️", "Settings, preferences"),
            ("x", "✕", "Close, cancel, dismiss"),
            ("chevron-right", "›", "Next, forward navigation"),
            ("chevron-left", "‹", "Back, previous navigation"),
            ("arrow-right", "→", "Directional indicator"),
            ("external-link", "↗", "Open in new window"),
        ];
        for (name, emoji, desc) in nav_icons {
            html.push_str(&format!(
                "- {} **[{}](https://lucide.dev/icons/{})** — {}\n",
                emoji, name, name, desc
            ));
        }

        html.push_str("\n## 👥 User & People\n\n");
        let user_icons = vec![
            ("user", "👤", "User profile, account"),
            ("users", "👥", "Multiple users, team"),
            ("user-plus", "➕👤", "Add user, invite"),
            ("user-check", "✓👤", "Verified user"),
            ("contact", "📇", "Contact information"),
        ];
        for (name, emoji, desc) in user_icons {
            html.push_str(&format!(
                "- {} **[{}](https://lucide.dev/icons/{})** — {}\n",
                emoji, name, name, desc
            ));
        }

        html.push_str("\n## 📁 Files & Folders\n\n");
        let file_icons = vec![
            ("file", "📄", "Generic file, document"),
            ("file-text", "📝", "Text document"),
            ("folder", "📁", "Directory, folder"),
            ("folder-open", "📂", "Opened folder"),
            ("download", "⬇️", "Download file"),
            ("upload", "⬆️", "Upload file"),
            ("save", "💾", "Save changes"),
        ];
        for (name, emoji, desc) in file_icons {
            html.push_str(&format!(
                "- {} **[{}](https://lucide.dev/icons/{})** — {}\n",
                emoji, name, name, desc
            ));
        }

        html.push_str("\n## 💬 Communication\n\n");
        let comm_icons = vec![
            ("mail", "📧", "Email, message"),
            ("message-circle", "💬", "Chat, conversation"),
            ("phone", "📞", "Phone call"),
            ("bell", "🔔", "Notifications, alerts"),
            ("send", "📤", "Send message"),
        ];
        for (name, emoji, desc) in comm_icons {
            html.push_str(&format!(
                "- {} **[{}](https://lucide.dev/icons/{})** — {}\n",
                emoji, name, name, desc
            ));
        }

        html.push_str("\n## 🎨 Media & Content\n\n");
        let media_icons = vec![
            ("image", "🖼️", "Image, picture"),
            ("camera", "📷", "Take photo"),
            ("video", "📹", "Video content"),
            ("music", "🎵", "Audio, music"),
            ("play", "▶", "Play media"),
            ("pause", "⏸", "Pause playback"),
        ];
        for (name, emoji, desc) in media_icons {
            html.push_str(&format!(
                "- {} **[{}](https://lucide.dev/icons/{})** — {}\n",
                emoji, name, name, desc
            ));
        }

        html.push_str("\n## 💡 Actions & Status\n\n");
        let action_icons = vec![
            ("check", "✅", "Success, complete, done"),
            ("x", "❌", "Error, cancel, close"),
            ("alert-circle", "⚠️", "Warning, caution"),
            ("info", "ℹ️", "Information, help"),
            ("heart", "❤️", "Like, favorite, love"),
            ("star", "⭐", "Favorite, rating"),
            ("trash", "🗑️", "Delete, remove"),
            ("edit", "✏️", "Edit, modify"),
            ("plus", "➕", "Add, create new"),
        ];
        for (name, emoji, desc) in action_icons {
            html.push_str(&format!(
                "- {} **[{}](https://lucide.dev/icons/{})** — {}\n",
                emoji, name, name, desc
            ));
        }

        html.push_str("\n## 🔒 Security\n\n");
        let security_icons = vec![
            ("lock", "🔒", "Locked, secure, private"),
            ("unlock", "🔓", "Unlocked, public"),
            ("key", "🔑", "Password, access"),
            ("shield", "🛡️", "Protection, security"),
            ("eye", "👁", "View, visible"),
            ("eye-off", "🙈", "Hidden, invisible"),
        ];
        for (name, emoji, desc) in security_icons {
            html.push_str(&format!(
                "- {} **[{}](https://lucide.dev/icons/{})** — {}\n",
                emoji, name, name, desc
            ));
        }

        html.push_str("\n## 📅 Time & Calendar\n\n");
        let time_icons = vec![
            ("calendar", "📅", "Calendar, date"),
            ("clock", "⏰", "Time, clock"),
            ("timer", "⏱️", "Stopwatch, countdown"),
        ];
        for (name, emoji, desc) in time_icons {
            html.push_str(&format!(
                "- {} **[{}](https://lucide.dev/icons/{})** — {}\n",
                emoji, name, name, desc
            ));
        }

        html.push_str("\n## ☁️ Cloud & Storage\n\n");
        let cloud_icons = vec![
            ("cloud", "☁️", "Cloud storage"),
            ("cloud-upload", "☁️⬆", "Upload to cloud"),
            ("cloud-download", "☁️⬇", "Download from cloud"),
            ("database", "🗄️", "Database, storage"),
        ];
        for (name, emoji, desc) in cloud_icons {
            html.push_str(&format!(
                "- {} **[{}](https://lucide.dev/icons/{})** — {}\n",
                emoji, name, name, desc
            ));
        }

        html.push_str("\n---\n\n");
        html.push_str("## 🚀 Quick Start Guide\n\n");
        html.push_str("### Installation\n\n");
        html.push_str("```bash\n");
        html.push_str("# React\n");
        html.push_str("npm install lucide-react\n\n");
        html.push_str("# Svelte\n");
        html.push_str("npm install lucide-svelte\n\n");
        html.push_str("# Vue\n");
        html.push_str("npm install lucide-vue-next\n");
        html.push_str("```\n\n");

        html.push_str("### Basic Usage\n\n");
        html.push_str("**React:**\n");
        html.push_str("```jsx\n");
        html.push_str("import { Home, User, Settings } from 'lucide-react';\n\n");
        html.push_str("<Home size={24} />\n");
        html.push_str("```\n\n");

        html.push_str("**Svelte:**\n");
        html.push_str("```svelte\n");
        html.push_str("import { Home } from 'lucide-svelte';\n\n");
        html.push_str("<Home size={24} />\n");
        html.push_str("```\n\n");

        html.push_str("**Vue:**\n");
        html.push_str("```vue\n");
        html.push_str("import { Home } from 'lucide-vue-next';\n\n");
        html.push_str("<Home :size=\"24\" />\n");
        html.push_str("```\n\n");

        html.push_str("**HTML:**\n");
        html.push_str("```html\n");
        html.push_str("<i data-lucide=\"home\"></i>\n\n");
        html.push_str("<script src=\"https://unpkg.com/lucide@latest\"></script>\n");
        html.push_str("<script>lucide.createIcons();</script>\n");
        html.push_str("```\n\n");

        html.push_str("---\n\n");
        html.push_str("## 💡 Tips for Using This Extension\n\n");
        html.push_str(
            "1. **Search by name:** `/lucide-search home` — Get detailed code examples\n",
        );
        html.push_str(
            "2. **Use tab completion:** Start typing after `/lucide-search` to see suggestions\n",
        );
        html.push_str(
            "3. **Browse categories:** `/lucide-browse` — View icons organized by purpose\n",
        );
        html.push_str("4. **Search by tags:** Icons are searchable by related terms (e.g., \"delete\" finds trash icon)\n\n");

        html.push_str("### 📚 Additional Resources\n\n");
        html.push_str("- [Official Lucide Website](https://lucide.dev) — Browse all 1640+ icons with live preview\n");
        html.push_str("- [React Documentation](https://lucide.dev/guide/packages/lucide-react)\n");
        html.push_str(
            "- [Svelte Documentation](https://lucide.dev/guide/packages/lucide-svelte)\n",
        );
        html.push_str("- [Vue Documentation](https://lucide.dev/guide/packages/lucide-vue-next)\n");
        html.push_str("- [GitHub Repository](https://github.com/lucide-icons/lucide)\n");

        html
    }
}

zed::register_extension!(LucideIconsExtension);
