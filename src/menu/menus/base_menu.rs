use crate::menu::menu_state::MenuState;

pub trait BaseMenu {
    fn run(&self) -> MenuState;
    fn get_choices(&self) -> &Vec<MenuState>;
}
