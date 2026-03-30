use crate::menu::menu_state::MenuState;

pub trait BaseMenu {
    fn display(&self);
    fn next_menu(&self) -> MenuState;
}
