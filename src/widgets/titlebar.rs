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
    title: String,
    tips: Option<TitleBarTips>,
}

impl Default for TitleBarState {
    fn default() -> Self {
        Self {
            title: "⚛ Elemental Minecraft Launcher".into(),
            tips: Some(TitleBarTips::new(
                "Press ``Ctrl+C`` to quit, use ``↑`` ``↓`` to browse, ``ENTER`` to focus the page, and ``ESC`` or ``q`` to return."
                    .into(),
            )),
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

        if let Some(tips) = &state.tips {
            tips.render(align(chunks[1], 1), buf);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum TipSegment {
    Plain(String),
    Code(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TitleBarTips {
    segments: Vec<TipSegment>,
}

impl TitleBarTips {
    pub fn new(text: String) -> Self {
        Self {
            segments: parse_tip_segments(&text),
        }
    }
}

fn parse_tip_segments(text: &str) -> Vec<TipSegment> {
    let mut segments = Vec::new();
    let mut cursor = 0;

    while let Some(open_offset) = text[cursor..].find("``") {
        let open_index = cursor + open_offset;

        if open_index > cursor {
            segments.push(TipSegment::Plain(text[cursor..open_index].to_string()));
        }

        let code_start = open_index + 2;
        let Some(close_offset) = text[code_start..].find("``") else {
            segments.push(TipSegment::Plain(text[open_index..].to_string()));
            return segments;
        };
        let code_end = code_start + close_offset;

        segments.push(TipSegment::Code(text[code_start..code_end].to_string()));
        cursor = code_end + 2;
    }

    if cursor < text.len() {
        segments.push(TipSegment::Plain(text[cursor..].to_string()));
    }

    segments
}

fn build_tip_line<'a>(segments: &'a [TipSegment]) -> Line<'a> {
    let normal_style = Style::default().fg(Color::LightYellow).italic();
    let code_style = Style::default().fg(Color::LightGreen).bold();
    let mut spans = Vec::with_capacity(segments.len() + 1);

    spans.push(Span::styled("Tips: ", normal_style));

    for segment in segments {
        match segment {
            TipSegment::Plain(text) => {
                spans.push(Span::styled(text.as_str(), normal_style));
            }
            TipSegment::Code(text) => {
                spans.push(Span::styled(text.as_str(), code_style));
            }
        }
    }

    Line::from(spans)
}

impl Widget for &TitleBarTips {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(build_tip_line(&self.segments)).render(area, buf);
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
