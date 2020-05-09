#[derive(Debug)]
pub struct Settings {
    /// Replace by unicode text characters
    pub unicode_replacement: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            unicode_replacement: false
        }
    }
}
