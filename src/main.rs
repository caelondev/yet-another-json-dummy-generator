use yajdg::menu::{Menu, menu_manager::MenuStateManager, menu_state::MenuState};

fn main() {
    loop {
        let mut menu_manager = MenuStateManager::new();
        let menu = Menu::from(*menu_manager.current_menu_state(), &menu_manager);

        menu.display();
        let next_state = menu.next_menu();

        if next_state == MenuState::Back {
            menu_manager.go_back();
            continue;
        }

        menu_manager.navigate_to(next_state);
    }
}
