#![no_std]
#![feature(start)]
utils::entry!(main);

use ufmt_stdio::*;

#[cfg(feature="to_int_unchecked")]
fn to_int(value: f32) -> i32 {
    unsafe {
        value.to_int_unchecked::<i32>()
    }
}

#[cfg(not(feature="to_int_unchecked"))]
fn to_int(value: f32) -> i32 {
    value as i32
}

fn main() {
    let pi = 4.0 * (0..666).map(|i| i * 2 + 1).map(|i| (1.0 - ((i & 2) as f32)) / (i as f32)).sum::<f32>();
    ufmt_stdio::println!("100 pi: {}", to_int(100.0 * pi));
}
