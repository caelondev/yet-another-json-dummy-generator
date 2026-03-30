use yajdg::menu::{Menu, menu_manager::MenuStateManager, menu_state::MenuState};

fn main() {
    let mut menu_manager = MenuStateManager::new();
    loop {
        let menu = Menu::from(*menu_manager.current_menu_state());
        let next_state = menu.run();

        if next_state == MenuState::Back {
            menu_manager.go_back();
            continue;
        }

        menu_manager.navigate_to(next_state);
    }
}
