use crate::menu::menu_state::MenuState;

pub fn ask(question: &str) -> String {
    use std::io::{self, Write};

    print!("{}", question);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn prompt_choice(choices: Vec<MenuState>) -> Option<MenuState> {
    let ans = ask("Enter your choice: ");
    if let Ok(num) = ans.parse::<usize>() {
        if num > 0 && num <= choices.len() {
            return Some(choices[num - 1].clone());
        }
    }

    None
}
