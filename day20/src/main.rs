#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
#[allow(unused_imports)]
use mos_alloc;
use ufmt_stdio::{ufmt::derive::uDebug, *};
use utils::to_str;

const MAX_SIZE: usize = 5000;

type Int = i16;

const KEY: i64 = 811589153;

fn mix_numbers(numbers: &mut Vec<(usize, Int)>, n: usize, key: i64) -> i64 {
    for _ in 0..n {
        for i in 0..numbers.len() {
            let index = numbers.iter().position(|v| v.0 == i).unwrap();
            let v = numbers.remove(index);
            let mut dst = (index as i64 + v.1 as i64 * key).rem_euclid(numbers.len() as i64);
            if dst == 0 {
                dst = numbers.len() as i64;
            }
            numbers.insert(dst as usize, v);
        }
    }
    let zero_index = numbers.iter().position(|v| v.1 == 0).unwrap();
    let v1 = numbers[(zero_index + 1000).rem_euclid(numbers.len())].1 as i64;
    let v2 = numbers[(zero_index + 2000).rem_euclid(numbers.len())].1 as i64;
    let v3 = numbers[(zero_index + 3000).rem_euclid(numbers.len())].1 as i64;
    key * (v1 + v2 + v3)
}

#[cfg_attr(not(test), export_name = "main")]
#[cfg_attr(test, allow(dead_code))]
fn main() -> isize {
    #[cfg(target_arch = "mos")]
    mos_alloc::set_limit(MAX_SIZE * 4 + 1000);
    let mut numbers = Vec::with_capacity(MAX_SIZE);
    let iter = utils::iter_lines!("../../input/day20/input.txt")
        .map(|v| to_str(v).parse::<Int>().unwrap());

    numbers.extend(iter.clone().enumerate());
    let part1 = mix_numbers(&mut numbers, 1, 1);
    assert!(part1 == 7278);
    println!("PART1: {}", part1);

    numbers.clear();
    numbers.extend(iter.enumerate());

    let part2 = mix_numbers(&mut numbers, 10, KEY);
    // assert!(part2 == 14375678667089);
    println!("PART2: {}", part2);
    return 0;
}
