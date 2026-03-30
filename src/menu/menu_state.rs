#[derive(Clone, Copy, PartialEq)]
pub enum MenuState {
    Main,
    Back,
    Exit,
}

impl MenuState {
    pub fn message(&self) {
        match self {
            MenuState::Main => println!("Main Menu"),
            MenuState::Back => println!("Back"),
            MenuState::Exit => println!("Exit"),
        }
    }
}
