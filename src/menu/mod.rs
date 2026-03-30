use crate::menu::{
    menu_state::MenuState,
    menus::{base_menu::BaseMenu, exit_menu::ExitMenu, main_menu::MainMenu},
};

pub mod menu_manager;
pub mod menu_state;
pub mod menus;

pub struct Menu;

impl Menu {
    pub fn from(state: MenuState) -> Box<dyn BaseMenu> {
        match state {
            MenuState::Main => Box::new(MainMenu::new()),
            MenuState::Exit => Box::new(ExitMenu::new()),
            MenuState::Back => unreachable!(),
        }
    }
}
