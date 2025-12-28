use crate::tui::theme::Theme;
use crate::tui::views::ViewMode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

pub struct StatusBar<'a> {
    view_mode: ViewMode,
    message: Option<&'a str>,
}

impl<'a> StatusBar<'a> {
    pub fn new(view_mode: ViewMode) -> Self {
        Self {
            view_mode,
            message: None,
        }
    }

    pub fn message(mut self, msg: Option<&'a str>) -> Self {
        self.message = msg;
        self
    }

    fn keybindings(&self) -> &'static str {
        match self.view_mode {
            ViewMode::Dashboard => {
                "q:quit  ←/→:harness  ↑/↓:profile  Enter:switch  n:new  d:del  e:edit  r:refresh  ?:help"
            }
            ViewMode::Legacy => {
                "q:quit  Tab:pane  ↑/↓:nav  Enter:switch  n:new  d:del  e:edit  r:refresh  ?:help"
            }
            #[cfg(feature = "tui-cards")]
            ViewMode::Cards => {
                "q:quit  ←/→:harness  ↑/↓:profile  Enter:switch  n:new  d:del  e:edit  r:refresh  ?:help"
            }
        }
    }

    fn mode_indicator(&self) -> &'static str {
        match self.view_mode {
            ViewMode::Dashboard => "[Dashboard]",
            ViewMode::Legacy => "[Legacy]",
            #[cfg(feature = "tui-cards")]
            ViewMode::Cards => "[Cards]",
        }
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let msg = self.message.unwrap_or("");

        let spans = vec![
            Span::styled(self.mode_indicator(), Theme::tab_selected()),
            Span::raw(" "),
            Span::styled(self.keybindings(), Theme::text_muted()),
            Span::raw("  "),
            Span::styled(msg, Theme::text_warning()),
        ];

        let paragraph = Paragraph::new(Line::from(spans));
        paragraph.render(area, buf);
    }
}
