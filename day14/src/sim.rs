use crate::display::DisplayInterface;

pub struct Display {
    memory: [u8; 8000], // 40 x 200 (bits/pixel)
}

impl DisplayInterface for Display {
    fn init() -> Self {
        Display { memory: [0; 8000] }
    }
    fn get_screen_memory(&mut self) -> *mut u8 {
        self.memory.as_mut_ptr()
    }
    fn show(&mut self) {}
    fn hide(&self) {}
}
