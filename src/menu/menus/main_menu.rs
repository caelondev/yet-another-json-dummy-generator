use crate::{
    menu::{menu_state::MenuState, menus::base_menu::BaseMenu},
    utils::print_choices,
};

pub struct MainMenu {
    choices: Vec<MenuState>,
}

impl BaseMenu for MainMenu {
    fn run(&self) -> MenuState {
        print_choices(&self.choices);
        return MenuState::Exit;
    }
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            // placeholder
            choices: vec![MenuState::Main, MenuState::Exit, MenuState::Back],
        }
    }
}
