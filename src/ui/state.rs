use egui_aesthetix::{themes::StandardLight, Aesthetix};
use std::rc::Rc;

// Applications pages.
#[derive(Debug, Default)]
pub enum Page {
    #[default]
    Main,
}

/// Application state.
#[derive(Debug)]
pub struct ApplicationState {
    // The currently selected page.
    pub active_page: Page,
    // The active theme.
    pub active_theme: Rc<dyn Aesthetix>,
}

impl Default for ApplicationState {
    fn default() -> ApplicationState {
        Self {
            active_page: Page::Main,
            active_theme: Rc::new(StandardLight),
        }
    }
}

impl ApplicationState {
    /// Create a new state with an active theme
    #[must_use]
    pub const fn new(active_theme: Rc<dyn Aesthetix>) -> Self {
        Self {
            active_page: Page::Main,
            active_theme,
        }
    }
}
