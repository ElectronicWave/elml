mod app;
mod i18n;
mod pages;
mod widgets;

use color_eyre::Result;

use crate::app::tui::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
