use crate::menu::menu_state::MenuState;

pub struct MenuManager {
    menus: Vec<MenuState>,
    current_menu_idx: usize,
}

impl MenuManager {
    pub fn new() -> Self {
        MenuManager {
            menus: vec![MenuState::Main],
            current_menu_idx: 0,
        }
    }

    pub fn current_menu(&self) -> &MenuState {
        &self.menus[self.current_menu_idx]
    }

    pub fn navigate_to(&mut self, menu: MenuState) {
        self.menus.push(menu);
        self.current_menu_idx += 1;
    }

    pub fn go_back(&mut self) {
        if self.current_menu_idx > 0 {
            self.menus.pop();
            self.current_menu_idx -= 1;
        }
    }
}
