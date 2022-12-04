#![no_std]
#![feature(start)]

utils::entry!(main);
use itertools::*;
use ufmt_stdio::*;

fn main() {
    let parsed = utils::iter_lines!("input.txt").map(|line| {
        line.split(|c| *c == b',')
            .map(|v| {
                v.split(|c| *c == b'-')
                    .map(|v| utils::to_str(v).parse::<u8>().unwrap())
                    .collect_tuple::<(_, _)>()
                    .unwrap()
            })
            .collect_tuple::<(_, _)>()
            .unwrap()
    });

    let part1 = parsed
        .clone()
        .filter(|((al, ah), (bl, bh))| al >= bl && ah <= bh || al <= bl && ah >= bh)
        .count();

    let part2 = parsed
        .filter(|((al, ah), (bl, bh))| !(ah < bl || bh < al))
        .count();

    assert!(part1 == 485);
    assert!(part2 == 857);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
