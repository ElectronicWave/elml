use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Spacing},
    symbols::merge::MergeStrategy,
    widgets::{Block, Borders, ListState},
};

use crate::widgets::{
    sidebar::{Sidebar, SidebarState},
    titlebar::TitleBar,
};

// Widget === Page === Route
pub struct Page {
    pub sidebar: SidebarState,
}

impl Page {
    pub fn new() -> Self {
        Self {
            sidebar: SidebarState {
                list: ListState::default().with_selected(Some(0)),
            },
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(15), Constraint::Fill(1)])
            .spacing(Spacing::Overlap(1))
            .split(frame.area());

        let titlebar_area = chunks[0];
        let content = chunks[1];

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .spacing(Spacing::Overlap(1))
            .constraints(vec![Constraint::Percentage(20), Constraint::Fill(1)])
            .split(content);
        let sidebar_area = chunks[0];
        let page_area = chunks[1];

        let titlebar = TitleBar::new("Elemental".into());
        let sidebar = Sidebar::new(vec![
            "Home".into(),
            "Instances".into(),
            "Catalog".into(),
            "Settings".into(),
        ]);
        frame.render_widget(titlebar, titlebar_area);
        frame.render_stateful_widget(sidebar, sidebar_area, &mut self.sidebar);
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .merge_borders(MergeStrategy::Exact),
            page_area,
        );
    }
}
