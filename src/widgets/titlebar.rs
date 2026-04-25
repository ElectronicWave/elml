use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    widgets::{Block, Borders, Widget},
};

pub struct TitleBar {
    text: String,
}

impl TitleBar {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl Widget for TitleBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .merge_borders(MergeStrategy::Exact);
        let inner = block.inner(area);
        block.render(area, buf);

        buf.set_string(
            inner.left() + 1,
            inner.y + (inner.height - 1) / 2,
            &self.text,
            Style::default().fg(Color::White),
        );
    }
}
