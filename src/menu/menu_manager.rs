use crate::menu::menu_state::MenuState;

pub struct MenuStateManager {
    menu_states: Vec<MenuState>,
    current_menu_idx: usize,
}

impl MenuStateManager {
    pub fn new() -> Self {
        Self {
            menu_states: vec![MenuState::Main],
            current_menu_idx: 0,
        }
    }

    pub fn current_menu_state(&self) -> &MenuState {
        &self.menu_states[self.current_menu_idx]
    }

    pub fn navigate_to(&mut self, menu: MenuState) {
        self.menu_states.push(menu);
        self.current_menu_idx += 1;
    }

    pub fn go_back(&mut self) {
        if self.current_menu_idx > 0 {
            self.menu_states.pop();
            self.current_menu_idx -= 1;
        }
    }
}
