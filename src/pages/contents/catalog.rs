use crate::{
    pages::content::{Content, container},
    widgets::{
        future::{FutureWidget, FutureWidgetState, Snapshot},
        selected::{SelectedScrollView, SelectedScrollViewState},
    },
};
use anyhow::Ok;
use elemental::{driver::drivers::vanilla::catalog::VanillaCatalog, launcher::Launcher};
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};

#[derive(Debug, Clone, Default)]
pub struct CatalogContent(CatalogContentState);

#[derive(Debug, Clone, Default)]
pub struct CatalogContentState {
    pub scroll: SelectedScrollViewState,
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
                    SelectedScrollView::new(&data, |_, version, is_selected, area, buf| {
                        let style = if is_selected {
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
                        paragraph.render(area, buf);
                    })
                    .render(area, buf, &mut self.0.scroll);
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
                        self.0.scroll.down(data.len());
                    } else if key.code == KeyCode::Up {
                        self.0.scroll.up();
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
