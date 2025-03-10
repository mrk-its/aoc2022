#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)] // https://github.com/rust-lang/rust/issues/133199#issuecomment-2630615573

extern crate alloc;
extern crate mos_alloc;

use alloc::vec::Vec;
use ufmt_stdio::*;
use utils::BitSet;

type Score = usize;

fn priority(v: u8) -> usize {
    match v {
        b'a'..=b'z' => (v - b'a' + 1) as usize,
        b'A'..=b'Z' => (v - b'A' + 27) as usize,
        _ => panic!(),
    }
}

fn intersect(lines: &[&[u8]]) -> Score {
    let mut out = BitSet::<64>::all_set();
    for data in lines[..lines.len() - 1].iter() {
        let mut tmp = BitSet::<64>::new();
        tmp.extend(data.iter().cloned().map(priority));
        out.intersect(&tmp);
        // for some reason calling this overloaded operator is slower :/
        // out &= tmp;
    }
    if let Some(data) = lines.last().cloned() {
        for v in data.iter().cloned().map(priority) {
            if out.contains(v) {
                return v as Score;
            }
        }
    }
    panic!();
}

fn score1(line: &[u8]) -> Score {
    let a = &line[0..line.len() / 2];
    let b = &line[line.len() / 2..];
    intersect(&[a, b])
}

#[cfg_attr(not(test), export_name = "main")]
#[cfg_attr(test, allow(dead_code))]
fn main() -> isize {
    let input = utils::iter_lines!("../../input/day03/input.txt").collect::<Vec<_>>();
    let part1 = input.iter().cloned().map(score1).sum::<Score>();
    assert!(part1 == 8153);
    println!("PART1: {}", part1);

    // TODO: use iter_array_chunks for no_alloc
    let part2 = input.chunks(3).map(intersect).sum::<Score>();
    assert!(part2 == 2342);
    println!("PART2: {}", part2);
    return 0;
}
