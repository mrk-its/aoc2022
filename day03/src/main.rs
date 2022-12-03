#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

utils::entry!(main);
use ufmt_stdio::*;
use utils::BitSet;
use alloc::vec::Vec;

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
    for data in lines.last().cloned() {
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

fn main() {
    let input = utils::iter_lines!("input.txt").collect::<Vec<_>>();
    let part1 = input.iter().cloned().map(score1).sum::<Score>();
    // TODO: use iter_array_chunks for no_alloc
    let part2 = input.chunks(3).map(intersect).sum::<Score>();

    assert!(part1 == 8153);
    assert!(part2 == 2342);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
