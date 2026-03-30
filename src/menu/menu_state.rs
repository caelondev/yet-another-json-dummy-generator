#[derive(Clone, Copy, PartialEq)]
pub enum MenuState {
    Main,
    Back,
    Exit,
}

impl MenuState {
    pub fn message(&self) -> &str {
        match self {
            MenuState::Main => "Main Menu",
            MenuState::Back => "Back",
            MenuState::Exit => "Exit",
        }
    }
}
