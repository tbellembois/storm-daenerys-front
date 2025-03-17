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
}

impl Default for ApplicationState {
    fn default() -> ApplicationState {
        Self {
            active_page: Page::Main,
        }
    }
}

impl ApplicationState {
    /// Create a new state with an active theme
    #[must_use]
    pub const fn new() -> Self {
        Self {
            active_page: Page::Main,
        }
    }
}
