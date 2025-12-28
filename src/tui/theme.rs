use ratatui::style::{Color, Modifier, Style};

/// Theme constants for consistent styling across the TUI.
#[allow(dead_code)]
pub struct Theme;

#[allow(dead_code)]
impl Theme {
    // Pane borders
    pub fn border_active() -> Style {
        Style::default().fg(Color::Cyan)
    }

    pub fn border_inactive() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    // Selection highlighting
    pub fn highlight() -> Style {
        Style::default()
            .fg(Color::White)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    }

    // Profile states
    pub fn profile_active() -> Style {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    }

    pub fn profile_normal() -> Style {
        Style::default()
    }

    // Harness states
    pub fn harness_installed() -> Style {
        Style::default()
    }

    pub fn harness_not_installed() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    // MCP server states
    pub fn mcp_enabled() -> Style {
        Style::default().fg(Color::Green)
    }

    pub fn mcp_disabled() -> Style {
        Style::default().fg(Color::Red)
    }

    // Text styles
    pub fn text_muted() -> Style {
        Style::default().add_modifier(Modifier::DIM)
    }

    pub fn text_gray() -> Style {
        Style::default().fg(Color::Gray)
    }

    pub fn text_warning() -> Style {
        Style::default().fg(Color::Yellow)
    }

    pub fn text_white() -> Style {
        Style::default().fg(Color::White)
    }

    // Tab styles
    pub fn tab_selected() -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }

    pub fn tab_normal() -> Style {
        Style::default().fg(Color::Gray)
    }

    // Help modal
    pub fn help_border() -> Style {
        Style::default().fg(Color::Cyan)
    }

    pub fn help_background() -> Style {
        Style::default().bg(Color::Black)
    }

    pub fn bold() -> Style {
        Style::default().add_modifier(Modifier::BOLD)
    }
}
