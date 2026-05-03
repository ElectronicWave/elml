use crate::{
    pages::content::{Content, container},
    widgets::future::{FutureWidget, FutureWidgetState, Snapshot},
};
use anyhow::Ok;
use elemental::{driver::drivers::vanilla::catalog::VanillaCatalog, launcher::Launcher};
use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Rect, Size},
    style::{Modifier, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

#[derive(Debug, Clone, Default)]
pub struct CatalogContent(CatalogContentState);

#[derive(Debug, Clone, Default)]
pub struct CatalogContentState {
    pub versions: VersionsScrollViewState,
    pub future: FutureWidgetState<Vec<VersionData>, anyhow::Error>,
}

impl Content for CatalogContent {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let inner = container(frame, area);
        let widget = FutureWidget::new()
            .factory(|| {
                Box::pin(async {
                    let launcher = Launcher::builder().build();
                    let data = launcher.catalog(VanillaCatalog::with_defaults()).await?;
                    Ok(data
                        .into_iter()
                        .map(|(version_id, releases)| VersionData {
                            version_id: version_id.to_string(),
                            description: releases
                                .first()
                                .and_then(|release| release.description.clone()),
                        })
                        .collect::<Vec<VersionData>>())
                })
            })
            .loading(|area, buf, _| {
                // Render loading state
                Paragraph::new("Loading...").centered().render(area, buf);
            })
            .ready(|area, buf, data| {
                // Render ready state with data
                let length = data.len();
                if length == 0 {
                    Paragraph::new("No versions found")
                        .centered()
                        .render(area, buf);
                } else {
                    VersionsScrollView(&data).render(area, buf, &mut self.0.versions);
                }
            })
            .error(|area, buf, error, previous| {
                // Render error state with error and optional previous data
                Paragraph::new(format!("Error: {}", error))
                    .centered()
                    .render(area, buf);
            });
        frame.render_stateful_widget(widget, inner, &mut self.0.future);
    }

    fn on_key(&mut self, key: KeyEvent) {
        match &**self.0.future.task.snapshot.load() {
            Snapshot::Ready(data) => {
                // Handle key events when the future widget is in the ready state
                // Control version list scrolling
                if key.is_press() {
                    if key.code == KeyCode::Down {
                        self.0.versions.scroll_down_with_protect(data.len());
                    } else if key.code == KeyCode::Up {
                        self.0.versions.scroll_up();
                    }
                }
            }
            _ => {
                // Handle key events based on the current state of the future widget
            }
        }
    }

    fn keybindings(&self) -> Option<Box<[(&'static str, &'static str)]>> {
        Some(Box::new([("ESC", "Return")]))
    }
}

#[derive(Debug, Clone, Default)]
pub struct VersionData {
    pub version_id: String,
    pub description: Option<String>,
}

pub struct VersionsScrollView<'a>(&'a [VersionData]);

#[derive(Debug, Clone, Default)]
pub struct VersionsScrollViewState {
    scroll: ScrollViewState,
    selected: usize,
}

impl VersionsScrollViewState {
    pub fn scroll_down_with_protect(&mut self, length: usize) {
        self.scroll.scroll_down();
        self.selected = self.selected.saturating_add(1).min(length - 1);
    }
    pub fn scroll_up(&mut self) {
        self.scroll.scroll_up();
        self.selected = self.selected.saturating_sub(1);
    }
}

impl<'a> StatefulWidget for VersionsScrollView<'a> {
    type State = VersionsScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let item_height = 1u16;
        let content_height = self.0.len() as u16 * item_height;

        let mut view = ScrollView::new(Size::new(area.width, content_height.max(area.height)))
            .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

        for (index, version) in self.0.iter().enumerate() {
            let y = index as u16 * item_height;

            let style = if index == state.selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            let text = match &version.description {
                Some(description) => {
                    format!("{} - {}", version.version_id, description)
                }

                None => version.version_id.clone(),
            };

            let paragraph = Paragraph::new(text).style(style);

            view.render_widget(paragraph, Rect::new(0, y, area.width.saturating_sub(1), 1));
        }

        view.render(area, buf, &mut state.scroll);
    }
}
