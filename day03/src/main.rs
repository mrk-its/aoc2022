#![no_std]
#![feature(start)]
utils::entry!(main);

use ufmt_stdio::*;
use utils::BitSet;

type Score = usize;

fn priority(v: u8) -> u8 {
    match v {
        b'a'..=b'z' => v - b'a' + 1,
        b'A'..=b'Z' => v - b'A' + 27,
        _ => panic!(),
    }
}

fn intersect(lines: &[&[u8]]) -> Score {
    let mut out = BitSet::<64>::new();
    for v in lines[0].iter().cloned() {
        out.insert(priority(v) as usize);
    }

    for data in lines[1..lines.len() - 1].iter().cloned() {
        let mut tmp = BitSet::<64>::new();
        for v in data.iter().cloned().map(priority) {
            tmp.insert(v as usize);
        }
        out.intersect(&tmp);
    }
    for data in lines.last().cloned() {
        for v in data.iter().cloned().map(priority) {
            if out.contains(v as usize) {
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
    let input = utils::iter_lines!("input.txt");
    let part1 = input.clone().map(score1).sum::<Score>();
    let mut part2 = 0;

    let mut index = 0;
    let mut group: [&[u8]; 3] = [b""; 3];

    for line in input {
        group[index] = line;
        index += 1;
        if index == 3 {
            index = 0;
            part2 += intersect(&group);
        }
    }

    assert!(part1 == 8153);
    assert!(part2 == 2342);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
