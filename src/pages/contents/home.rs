use std::pin::Pin;

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
};
use tui_scrollview::ScrollViewState;

use crate::pages::content::{Content, container};
#[derive(Debug, Clone, Default)]
pub struct HomeContent(HomeContentState);
#[derive(Debug, Clone, Default)]
pub struct HomeContentState {
    scroll: ScrollViewState,
    focus: usize,
}

pub enum HomeContentWidgets {}

impl Content for HomeContent {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let inner = container(frame, area);
    }

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down => self.0.scroll.scroll_down(),
            KeyCode::Up => self.0.scroll.scroll_up(),
            _ => {}
        }
    }

    fn keybindings(&self) -> Option<Box<[(&'static str, &'static str)]>> {
        Some(Box::new([("↑/↓", "Scroll"), ("ESC", "Return")]))
    }
}
