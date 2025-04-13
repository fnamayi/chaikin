use std::time::Instant;

/// Models a notification toast to be shown to the user
pub struct Toast {
    /// The toast message
    pub message: String,
    /// The instant when the toast was first shown
    pub shown_since: Option<Instant>,
}

impl Toast {
    /// Create a new toast
    pub fn new() -> Self {
        Toast {
            message: String::new(),
            shown_since: None,
        }
    }

    /// Show the given message in the toast notification
    pub fn show(&mut self, message: &str) {
        self.message = message.to_string();
        self.shown_since = Some(Instant::now());
    }

    /// Dismiss the toast
    pub fn dismiss(&mut self) {
        self.shown_since = None;
    }

    /// Returns whether the toast is still active
    pub fn is_showing(&self) -> bool {
        self.shown_since
            .map_or(false, |time| time.elapsed() < crate::window::TOAST_DURATION)
    }
}
