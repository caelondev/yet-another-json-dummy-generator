pub trait BaseMenu {
    fn display(&self);
    fn next_menu(&self) -> Box<dyn BaseMenu>;
}
