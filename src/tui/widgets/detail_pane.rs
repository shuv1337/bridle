use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::config::{McpServerInfo, ProfileInfo, ResourceSummary};

pub struct DetailPane<'a> {
    profile: Option<&'a ProfileInfo>,
    is_focused: bool,
}

impl<'a> DetailPane<'a> {
    pub fn new(profile: Option<&'a ProfileInfo>) -> Self {
        Self {
            profile,
            is_focused: false,
        }
    }

    #[allow(dead_code)]
    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    fn render_profile_details(profile: &ProfileInfo) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        let active_marker = if profile.is_active { "● " } else { "  " };
        lines.push(Line::from(vec![
            Span::styled(
                format!("{}{}", active_marker, profile.name),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " ─────────────────────────",
                Style::default().fg(Color::Gray),
            ),
        ]));

        #[derive(Clone)]
        enum Section<'a> {
            Theme(&'a str),
            Model(&'a str),
            Mcp(&'a [McpServerInfo]),
            Resources(&'static str, &'a ResourceSummary),
            Rules(&'a std::path::Path),
            Error(&'a str),
        }

        let mut sections: Vec<Section> = Vec::new();

        if let Some(ref theme) = profile.theme {
            sections.push(Section::Theme(theme));
        }
        if let Some(ref model) = profile.model {
            sections.push(Section::Model(model));
        }
        if !profile.mcp_servers.is_empty() {
            sections.push(Section::Mcp(&profile.mcp_servers));
        }
        if let Some(ref plugins) = profile.plugins {
            sections.push(Section::Resources("Plugins", plugins));
        }
        if let Some(ref agents) = profile.agents {
            sections.push(Section::Resources("Agents", agents));
        }
        sections.push(Section::Resources("Skills", &profile.skills));
        sections.push(Section::Resources("Commands", &profile.commands));
        if let Some(ref rules) = profile.rules_file {
            sections.push(Section::Rules(rules));
        }
        for error in &profile.extraction_errors {
            sections.push(Section::Error(error));
        }

        let total = sections.len();
        for (idx, section) in sections.iter().enumerate() {
            let is_last = idx == total - 1;
            let branch = if is_last { "└─" } else { "├─" };
            let cont = if is_last { "   " } else { "│  " };

            match section {
                Section::Theme(theme) => {
                    lines.push(Line::styled(
                        format!("  {} Theme: {}", branch, theme),
                        Style::default().fg(Color::Gray),
                    ));
                }
                Section::Model(model) => {
                    lines.push(Line::styled(
                        format!("  {} Model: {}", branch, model),
                        Style::default().fg(Color::Gray),
                    ));
                }
                Section::Mcp(servers) => {
                    lines.push(Line::styled(
                        format!("  {} MCP ({})", branch, servers.len()),
                        Style::default().fg(Color::Gray),
                    ));
                    let server_count = servers.len();
                    for (i, server) in servers.iter().enumerate() {
                        let sub_branch = if i == server_count - 1 {
                            "└─"
                        } else {
                            "├─"
                        };
                        let (marker, color) = if server.enabled {
                            ("✓", Color::Green)
                        } else {
                            ("✗", Color::Red)
                        };
                        let detail = match (&server.server_type, &server.command, &server.url) {
                            (Some(t), Some(cmd), _) => {
                                let args_str = server
                                    .args
                                    .as_ref()
                                    .map(|a| a.join(" "))
                                    .unwrap_or_default();
                                if args_str.is_empty() {
                                    format!(" ({t}): {cmd}")
                                } else {
                                    format!(" ({t}): {cmd} {args_str}")
                                }
                            }
                            (Some(t), None, Some(url)) => format!(" ({t}): {url}"),
                            (Some(t), None, None) => format!(" ({t})"),
                            _ => String::new(),
                        };
                        lines.push(Line::from(vec![
                            Span::styled(
                                format!("  {} {} ", cont, sub_branch),
                                Style::default().fg(Color::Gray),
                            ),
                            Span::styled(
                                format!("{} {}", marker, server.name),
                                Style::default().fg(color),
                            ),
                            Span::styled(detail, Style::default().fg(Color::DarkGray)),
                        ]));
                    }
                }
                Section::Resources(label, summary) => {
                    lines.push(Line::styled(
                        format!("  {} {} ({})", branch, label, summary.items.len()),
                        Style::default().fg(Color::Gray),
                    ));
                    if summary.items.is_empty() {
                        if !summary.directory_exists {
                            lines.push(Line::styled(
                                format!("  {} └─ (directory not found)", cont),
                                Style::default().fg(Color::Yellow),
                            ));
                        }
                    } else {
                        let item_count = summary.items.len();
                        for (i, item) in summary.items.iter().enumerate() {
                            let sub_branch = if i == item_count - 1 {
                                "└─"
                            } else {
                                "├─"
                            };
                            lines.push(Line::styled(
                                format!("  {} {} {}", cont, sub_branch, item),
                                Style::default().fg(Color::Gray),
                            ));
                        }
                    }
                }
                Section::Rules(path) => {
                    if let Some(filename) = path.file_name() {
                        lines.push(Line::styled(
                            format!("  {} Rules: {}", branch, filename.to_string_lossy()),
                            Style::default().fg(Color::Gray),
                        ));
                    }
                }
                Section::Error(error) => {
                    lines.push(Line::styled(
                        format!("  {} ⚠ {}", branch, error),
                        Style::default().fg(Color::Yellow),
                    ));
                }
            }
        }

        lines
    }
}

impl Widget for DetailPane<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.is_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(" Details ")
            .borders(Borders::ALL)
            .border_style(border_style);

        let content = match self.profile {
            Some(profile) => Self::render_profile_details(profile),
            None => vec![Line::styled(
                "Select a profile to view details",
                Style::default().fg(Color::DarkGray),
            )],
        };

        Paragraph::new(content).block(block).render(area, buf);
    }
}
