#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Legacy,
    #[default]
    Dashboard,
}

impl ViewMode {
    pub fn toggle(&mut self) {
        *self = match self {
            ViewMode::Legacy => ViewMode::Dashboard,
            ViewMode::Dashboard => ViewMode::Legacy,
        };
    }

    pub fn name(&self) -> &'static str {
        match self {
            ViewMode::Legacy => "Legacy",
            ViewMode::Dashboard => "Dashboard",
        }
    }
}
