use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect, Spacing},
};

use crate::pages::content::{ContentState, sidebar_entries};
use crate::widgets::{
    sidebar::{Sidebar, SidebarState},
    titlebar::{TitleBar, TitleBarState},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PageControlMode {
    Sidebar,
    Content,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub titlebar: TitleBarState,
    pub sidebar: SidebarState,
    pub control_mode: PageControlMode,
    pub content: ContentState,
}

impl Page {
    pub fn sidebar_entry_count(&self) -> usize {
        let entry_count = sidebar_entries().len();

        if entry_count == 0 {
            panic!("Sidebar should contain at least one entry");
        }

        entry_count
    }

    pub fn selected_sidebar_index(&self) -> usize {
        let selected_index = self.sidebar.list.selected().unwrap_or(0);
        let last_index = self.sidebar_entry_count() - 1;

        selected_index.min(last_index)
    }

    pub fn selected_sidebar_entry(&self) -> &'static crate::pages::content::SidebarEntry {
        &sidebar_entries()[self.selected_sidebar_index()]
    }

    pub fn normalize_sidebar_selection(&mut self) {
        let selected_index = self.selected_sidebar_index();
        self.sidebar.list.select(Some(selected_index));
    }

    pub fn select_next_sidebar_item(&mut self) {
        let selected_index = self.selected_sidebar_index();
        let last_index = self.sidebar_entry_count() - 1;
        let next_index = selected_index.saturating_add(1).min(last_index);

        self.sidebar.list.select(Some(next_index));
    }

    pub fn select_previous_sidebar_item(&mut self) {
        let selected_index = self.selected_sidebar_index();
        let previous_index = selected_index.saturating_sub(1);

        self.sidebar.list.select(Some(previous_index));
    }

    pub fn enter_content_control(&mut self) {
        self.control_mode = PageControlMode::Content;
    }

    pub fn exit_content_control(&mut self) {
        self.control_mode = PageControlMode::Sidebar;
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if !key.is_press() {
            return;
        }

        match self.control_mode {
            PageControlMode::Sidebar => match key.code {
                KeyCode::Down => self.select_next_sidebar_item(),
                KeyCode::Up => self.select_previous_sidebar_item(),
                KeyCode::Enter => {
                    self.enter_content_control();
                    self.titlebar.keybindings =
                        self.content.keybindings(self.selected_sidebar_entry().id);
                }
                _ => {}
            },
            PageControlMode::Content => match key.code {
                KeyCode::Esc => {
                    self.exit_content_control();
                    self.titlebar.keybindings.take();
                }
                _ => self.content.on_key(self.selected_sidebar_entry().id, key),
            },
        }
    }

    pub fn sidebar_items(&self) -> Vec<String> {
        sidebar_entries()
            .iter()
            .map(|entry| entry.label.to_string())
            .collect()
    }

    pub fn render_active_page(&mut self, frame: &mut Frame, area: Rect) {
        self.content
            .render(self.selected_sidebar_entry().id, frame, area);
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.normalize_sidebar_selection();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(4), Constraint::Fill(1)])
            .spacing(Spacing::Overlap(1))
            .split(frame.area());

        let titlebar_area = chunks[0];
        let content_area = chunks[1];

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .spacing(Spacing::Overlap(1))
            .constraints(vec![Constraint::Max(20), Constraint::Fill(1)])
            .split(content_area);
        let sidebar_area = chunks[0];
        let page_area = chunks[1];

        let titlebar = TitleBar;
        let sidebar = Sidebar::new(self.sidebar_items());

        frame.render_stateful_widget(titlebar, titlebar_area, &mut self.titlebar);
        frame.render_stateful_widget(sidebar, sidebar_area, &mut self.sidebar);
        self.render_active_page(frame, page_area);
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            titlebar: TitleBarState::default(),
            sidebar: SidebarState::default(),
            control_mode: PageControlMode::Sidebar,
            content: ContentState::default(),
        }
    }
}
