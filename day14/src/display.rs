pub trait DisplayInterface {
    fn init() -> Self;
    fn get_screen_memory(&mut self) -> *mut u8;
    fn show(&mut self);
    fn hide(&self);
    fn clear_atract(&self) {}
}
