#![no_std]
#![no_main]

use ufmt_stdio::*;

#[allow(unused_imports)]
use utils;

fn all_different(inp: &[u8]) -> bool {
    for i in 1..inp.len() {
        for j in 0..i {
            if inp[i] == inp[j] {
                return false;
            };
        }
    }
    return true;
}

fn start_of_packet(input: &[u8], n: usize) -> Option<usize> {
    for i in 0..input.len() - n {
        if all_different(&input[i..i + n]) {
            return Some(i + n);
        }
    }
    None
}

#[cfg_attr(not(test), export_name = "main")]
#[cfg_attr(test, allow(dead_code))]
fn main() -> isize {
    let input = include_bytes!("../../input/day06/input.txt");

    let part1 = start_of_packet(input, 4).unwrap();
    assert!(part1 == 1300);
    println!("PART1: {}", part1);

    let part2 = start_of_packet(input, 14).unwrap();
    assert!(part2 == 3986);
    println!("PART2: {}", part2);
    return 0;
}
