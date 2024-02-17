#![no_std]
#![feature(start)]
utils::entry!(main);

use ufmt_stdio::*;

fn main() {
    let pi = 4.0 * (0..666).map(|i| i * 2 + 1).map(|i| (1.0 - ((i & 2) as f32)) / (i as f32)).sum::<f32>();
    ufmt_stdio::println!("100 pi: {}", (100.0 * pi) as i32);
}
