use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::merge::MergeStrategy,
    widgets::{Block, Borders, List, ListState, StatefulWidget},
};

pub struct Sidebar {
    items: Vec<String>,
}

impl Sidebar {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }
}

pub struct SidebarState {
    pub list: ListState,
}

impl StatefulWidget for Sidebar {
    type State = SidebarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let list = List::new(self.items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .merge_borders(MergeStrategy::Exact),
            )
            .style(Style::new().white())
            .highlight_style(Style::new().italic().light_green())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        list.render(area, buf, &mut state.list);
    }
}
