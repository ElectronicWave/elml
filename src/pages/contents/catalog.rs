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
    widgets::{Paragraph, StatefulWidget, Widget},
};
use tui_scrollview::{ScrollView, ScrollViewState};

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
                    let text = data
                        .iter()
                        .map(|v| match &v.description {
                            Some(description) => format!("{} - {}", v.version_id, description),
                            None => v.version_id.clone(),
                        })
                        .collect::<Vec<String>>()
                        .join("\n");
                    Paragraph::new(text).render(area, buf);
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
            Snapshot::Ready(_) => {
                // Handle key events when the future widget is in the ready state
                // Control version list scrolling
                if key.is_press() {
                    if key.code == KeyCode::Down {
                        self.0.versions.scroll.scroll_down();
                    } else if key.code == KeyCode::Up {
                        self.0.versions.scroll.scroll_up();
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

pub struct VersionsScrollView;
#[derive(Debug, Clone, Default)]
pub struct VersionsScrollViewState {
    pub versions: Vec<VersionData>,
    pub scroll: ScrollViewState,
}
impl StatefulWidget for VersionsScrollView {
    type State = VersionsScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut view = ScrollView::new(Size::new(area.width, state.versions.len() as u16));

        view.render(area, buf, &mut state.scroll);
    }
}
