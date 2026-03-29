use crate::menu::{menu_state::MenuState, menus::base_menu::BaseMenu};

pub struct MainMenu {
    choices: Vec<MenuState>,
}

impl BaseMenu for MainMenu {
    fn display(&self) {
        todo!()
    }

    fn next_menu(&self) -> Box<dyn BaseMenu> {
        todo!()
    }
}
