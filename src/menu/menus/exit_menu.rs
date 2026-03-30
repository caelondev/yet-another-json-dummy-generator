use std::process;

use crate::menu::{menu_state::MenuState, menus::base_menu::BaseMenu};

pub struct ExitMenu;

impl BaseMenu for ExitMenu {
    fn run(&self) -> MenuState {
        println!("exiting");
        process::exit(0);
    }
}

impl ExitMenu {
    pub fn new() -> Self {
        Self
    }
}
