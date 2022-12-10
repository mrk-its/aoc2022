use crate::display::DisplayInterface;

pub struct Display;

impl DisplayInterface for Display {
    fn init(_width: usize, _height: usize, _ptr: *const u8) -> Self {
        Display
    }
    fn show(&mut self) {}
    fn scroll_to(&mut self, _x: usize, _y: usize) {}
    fn hide(&self) {}
}
