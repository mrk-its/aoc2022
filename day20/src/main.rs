#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

utils::entry!(main);
extern crate alloc;

use alloc::vec::Vec;
#[allow(unused_imports)]
use mos_alloc;
use ufmt_stdio::{ufmt::derive::uDebug, *};
use utils::to_str;

const MAX_SIZE: usize = 5000;

type Int = i64;

fn main() {
    #[cfg(target_arch = "mos")]
    mos_alloc::set_limit(MAX_SIZE * 4 + 1000);
    let mut numbers = Vec::with_capacity(MAX_SIZE);

    numbers.extend(
        utils::iter_lines!("../../input/day20/input.txt")
            .map(|v| to_str(v).parse::<Int>().unwrap())
            .enumerate(),
    );
    for i in 0..numbers.len() {
        let index = numbers.iter().position(|v| v.0 == i).unwrap();
        let v = numbers.remove(index);
        println!("v: {}", v.1);
        let mut dst = (index as Int + v.1).rem_euclid(numbers.len() as Int);
        if dst == 0 {
            dst = numbers.len() as Int;
        }
        numbers.insert(dst as usize, v);
    }
    let zero_index = numbers.iter().position(|v| v.1 == 0).unwrap();
    let v1 = numbers[(zero_index + 1000) % numbers.len()].1;
    let v2 = numbers[(zero_index + 2000) % numbers.len()].1;
    let v3 = numbers[(zero_index + 3000) % numbers.len()].1;
    println!("{} {} {} {}", v1, v2, v3, v1 + v2 + v3);
}
