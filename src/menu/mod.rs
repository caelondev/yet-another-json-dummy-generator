use crate::menu::{
    menu_manager::MenuStateManager,
    menu_state::MenuState,
    menus::{base_menu::BaseMenu, main_menu::MainMenu},
};

pub mod menu_manager;
pub mod menu_state;
pub mod menus;

pub struct Menu;

impl Menu {
    pub fn from(state: MenuState, manager: &MenuStateManager) -> Box<dyn BaseMenu> {
        match state {
            MenuState::Main => Box::new(MainMenu::new()),
            _ => todo!(),
        }
    }
}
