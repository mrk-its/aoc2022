#![no_std]
#![feature(start)]

utils::entry!(main);

extern "C" {
    fn __putchar(c: u8);
}

const DATA: &[u8] = b"ab";
fn main() {
    let mut iter = DATA.iter().cycle();
    for _ in 0..4 {
        unsafe { __putchar(*iter.next().unwrap()) };
    }
}
