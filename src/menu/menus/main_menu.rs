use crate::menu::{menu_state::MenuState, menus::base_menu::BaseMenu};

pub struct MainMenu {
    choices: Vec<MenuState>,
}

impl BaseMenu for MainMenu {
    fn display(&self) {
        println!("Test");
    }

    fn next_menu(&self) -> MenuState {
        return MenuState::Back;
    }
}

impl MainMenu {
    pub fn new() -> Self {
        Self { choices: vec![] }
    }
}
