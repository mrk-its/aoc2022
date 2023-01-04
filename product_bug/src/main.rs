#![no_std]
#![feature(start)]
use itertools::iproduct;

utils::entry!(main);

// const X: [u8; 2] = [b'1', b'2'];
// const Y: [u8; 3] = [b'A', b'B', b'C'];

const X: &[u8] = b"12";
const Y: &[u8] = b"ABC";

extern "C" {
    fn __putchar(c: u8);
}

fn main() {
    for (x, y) in iproduct!(X, Y) {
        unsafe {
            __putchar(*x);
            __putchar(*y);
            __putchar(b'\n');
        }
    }
}
