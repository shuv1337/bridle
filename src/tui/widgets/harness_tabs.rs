use crate::harness::HarnessConfig;
use harness_locate::{Harness, HarnessKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs, Widget},
};

pub struct HarnessTabs<'a> {
    harnesses: &'a [HarnessKind],
    selected: usize,
    statuses: Vec<(char, bool)>,
}

impl<'a> HarnessTabs<'a> {
    pub fn new(harnesses: &'a [HarnessKind], selected: usize) -> Self {
        let statuses = harnesses
            .iter()
            .map(|kind| {
                let harness = Harness::new(*kind);
                let installed = harness.is_installed();
                let indicator = if installed { '+' } else { ' ' };
                (indicator, installed)
            })
            .collect();

        Self {
            harnesses,
            selected,
            statuses,
        }
    }

    pub fn with_active_indicator(mut self, harness_id: &str, has_active: bool) -> Self {
        for (i, kind) in self.harnesses.iter().enumerate() {
            let h = Harness::new(*kind);
            if h.id() == harness_id && has_active {
                self.statuses[i].0 = '*';
            }
        }
        self
    }
}

impl Widget for HarnessTabs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles: Vec<Line> = self
            .harnesses
            .iter()
            .zip(self.statuses.iter())
            .map(|(kind, (indicator, installed))| {
                let harness = Harness::new(*kind);
                let name = harness.kind().to_string();
                let style = if *installed {
                    Style::default()
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                Line::from(vec![
                    Span::styled(format!("{} ", indicator), style),
                    Span::styled(name, style),
                ])
            })
            .collect();

        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .title(" Harnesses ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .select(self.selected)
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )
            .divider(Span::raw(" │ "));

        tabs.render(area, buf);
    }
}
