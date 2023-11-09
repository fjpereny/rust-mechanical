pub enum Popup {
    None,
    CloseWarning,
}

pub struct QuitWarningPopup {
    pub quit_button_selected: bool,
}

impl QuitWarningPopup {
    pub fn new() -> Self {
        QuitWarningPopup {
            quit_button_selected: false,
        }
    }
}
