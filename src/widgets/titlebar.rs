use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

#[derive(Debug, Clone, Default)]
pub struct TitleBar;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TitleBarState {
    pub title: String,
    pub keybindings: Option<Box<[(&'static str, &'static str)]>>,
}

impl Default for TitleBarState {
    fn default() -> Self {
        Self {
            title: "⚛  Elemental Minecraft Launcher".into(),
            keybindings: None,
        }
    }
}

impl StatefulWidget for TitleBar {
    type State = TitleBarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::ALL)
            .merge_borders(MergeStrategy::Exact);
        let inner = block.inner(area);

        block.render(area, buf);
        // we have enough space to display tips, so split title and tips into 2L
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner);

        buf.set_string(
            chunks[0].left() + 1,
            align(chunks[0], 1).y,
            &state.title,
            Style::default().fg(Color::White).bold(),
        );

        let keybindings = TitleBarKeybindings::with_default(state.keybindings.as_deref());
        keybindings.render(align(chunks[1], 1), buf);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TitleBarKeybindings<'a> {
    segments: &'a [(&'a str, &'a str)],
}
const DEFAULT_KEYBINDINGS: &[(&str, &str)] =
    &[("Ctrl+Q", "Quit"), ("↑/↓", "Browse"), ("ENTER", "Focus")];

impl<'a> Default for TitleBarKeybindings<'a> {
    fn default() -> Self {
        Self::new(DEFAULT_KEYBINDINGS)
    }
}

impl<'a> TitleBarKeybindings<'a> {
    pub fn new(keybindings: &'a [(&'a str, &'a str)]) -> Self {
        Self {
            segments: keybindings,
        }
    }

    pub fn with_default(keybindings: Option<&'a [(&'a str, &'a str)]>) -> Self {
        let segments = keybindings.unwrap_or(DEFAULT_KEYBINDINGS);
        if segments.is_empty() {
            return Self::default();
        }
        Self { segments }
    }
}

fn render_keybindings_spans<'a>(keybindings: &'a [(&'a str, &'a str)]) -> Vec<Span<'a>> {
    keybindings
        .iter()
        .flat_map(|(key, desc)| {
            let key_str = format!("[{}]", key);
            vec![
                Span::styled(key_str, Style::default().fg(Color::LightGreen).bold()),
                Span::raw(format!("{} ", desc)),
            ]
        })
        .collect()
}

impl<'a> Widget for &TitleBarKeybindings<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Line::from(render_keybindings_spans(self.segments))).render(area, buf);
    }
}

pub fn align(rect: Rect, height: u16) -> Rect {
    let y = if rect.height >= height {
        rect.y + (rect.height - 1) / 2
    } else {
        0
    };

    Rect {
        x: rect.left() + 1,
        y,
        width: rect.width,
        height: rect.height,
    }
}
