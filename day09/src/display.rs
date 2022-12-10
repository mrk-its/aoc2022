pub trait DisplayInterface {
    fn init(width: usize, height: usize, ptr: *const u8) -> Self;
    fn show(&mut self);
    fn scroll_to(&mut self, x: usize, y: usize);
    fn hide(&self);
}
