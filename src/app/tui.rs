use std::time::Duration;

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
};

use crate::pages::page::Page;

pub struct App {
    page: Page,
}

impl App {
    pub fn new() -> Self {
        Self { page: Page::new() }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break;
                    }

                    self.on_key(key);
                }
            }
        }
        Ok(())
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        // Match state
        // match self.page.sidebar.list.selected() {}
        if key.is_press() {
            match key.code {
                KeyCode::Down => self.page.sidebar.list.select_next(),
                KeyCode::Up => self.page.sidebar.list.select_previous(),
                _ => (),
            }
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.page.render(frame);
    }
}
