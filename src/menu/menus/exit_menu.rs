use std::process;

use crate::menu::{menu_state::MenuState, menus::base_menu::BaseMenu};

pub struct ExitMenu {
    choices: Vec<MenuState>,
}

impl BaseMenu for ExitMenu {
    fn run(&self) -> MenuState {
        println!("exiting");
        process::exit(0);
    }
    fn get_choices(&self) -> &Vec<MenuState> {
        &self.choices
    }
}

impl ExitMenu {
    pub fn new() -> Self {
        Self { choices: vec![] }
    }
}
