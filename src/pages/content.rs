use ratatui::{
    Frame,
    crossterm::event::KeyEvent,
    layout::Rect,
    style::Style,
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PageId {
    Home,
    Instances,
    Catalog,
    Settings,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct SidebarEntry {
    pub id: PageId,
    pub label: &'static str,
}

impl SidebarEntry {
    pub const fn new(id: PageId, label: &'static str) -> Self {
        Self { id, label }
    }
}

const SIDEBAR_ENTRIES: [SidebarEntry; 4] = [
    SidebarEntry::new(PageId::Home, "Home"),
    SidebarEntry::new(PageId::Instances, "Instances"),
    SidebarEntry::new(PageId::Catalog, "Catalog"),
    SidebarEntry::new(PageId::Settings, "Settings"),
];

pub fn sidebar_entries() -> &'static [SidebarEntry] {
    &SIDEBAR_ENTRIES
}

pub trait Content {
    fn render(&mut self, frame: &mut Frame, area: Rect);

    fn on_key(&mut self, key: KeyEvent);
}

#[derive(Debug, Clone, Default)]
pub struct ContentState {
    pub home: HomeContent,
    pub instances: InstancesContent,
    pub catalog: CatalogContent,
    pub settings: SettingsContent,
}

impl ContentState {
    pub fn render(&mut self, page_id: PageId, frame: &mut Frame, area: Rect) {
        match page_id {
            PageId::Home => self.home.render(frame, area),
            PageId::Instances => self.instances.render(frame, area),
            PageId::Catalog => self.catalog.render(frame, area),
            PageId::Settings => self.settings.render(frame, area),
        }
    }

    pub fn on_key(&mut self, page_id: PageId, key: KeyEvent) {
        match page_id {
            PageId::Home => self.home.on_key(key),
            PageId::Instances => self.instances.on_key(key),
            PageId::Catalog => self.catalog.on_key(key),
            PageId::Settings => self.settings.on_key(key),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct HomeContent;

impl Content for HomeContent {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        render_placeholder(
            frame,
            area,
            "Home",
            "This page can grow into a dashboard component with its own layout and state.",
        );
    }

    fn on_key(&mut self, _key: KeyEvent) {}
}

#[derive(Debug, Clone, Default)]
pub struct InstancesContent;

impl Content for InstancesContent {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        render_placeholder(
            frame,
            area,
            "Instances",
            "This page is ready for an instances list, actions, and per-instance details.",
        );
    }

    fn on_key(&mut self, _key: KeyEvent) {}
}

#[derive(Debug, Clone, Default)]
pub struct CatalogContent;

impl Content for CatalogContent {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        render_placeholder(
            frame,
            area,
            "Catalog",
            "This page can render catalog search, filters, and item detail widgets.",
        );
    }

    fn on_key(&mut self, _key: KeyEvent) {}
}

#[derive(Debug, Clone, Default)]
pub struct SettingsContent;

impl Content for SettingsContent {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        render_placeholder(
            frame,
            area,
            "Settings",
            "This page can render forms, tabs, and validation state without changing sidebar storage.",
        );
    }

    fn on_key(&mut self, _key: KeyEvent) {}
}

fn render_placeholder(frame: &mut Frame, area: Rect, title: &str, description: &str) {
    let lines = vec![
        Line::styled(title, Style::new().bold().white()),
        Line::raw(""),
        Line::raw(description),
    ];

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .merge_borders(MergeStrategy::Exact),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
