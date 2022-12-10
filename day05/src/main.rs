#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

utils::entry!(main);

use alloc::vec::Vec;
use itertools::*;
use ufmt_stdio::*;
use utils::to_str;

fn parse_cranes<'a>(input: &mut (impl Iterator<Item = &'a [u8]> + Clone)) -> Vec<Vec<u8>> {
    let mut max_items = 0;
    let mut n_cranes = 0;
    for line in input.clone() {
        if !line[1].is_ascii_digit() {
            max_items += 1;
        } else {
            n_cranes = line.len() / 4 + 1;
            break;
        }
    }
    let mut cranes: Vec<Vec<u8>> = Vec::with_capacity(n_cranes);
    cranes.resize_with(n_cranes, || Vec::with_capacity(n_cranes * max_items));

    for line in input.by_ref().take_while(|v| !v[1].is_ascii_digit()) {
        for (i, chunk) in line.chunks(4).enumerate() {
            if chunk[1].is_ascii_alphabetic() {
                cranes[i].push(chunk[1]);
            }
        }
    }

    for crane in cranes.iter_mut() {
        crane.reverse();
    }
    input.next();

    cranes
}

fn top_items(cranes: &Vec<Vec<u8>>) -> Vec<u8> {
    cranes
        .iter()
        .map(|crane| crane.last().unwrap())
        .cloned()
        .collect::<Vec<_>>()
}

fn main() {
    let mut input = utils::iter_lines!("../../input/day05/input.txt");
    let mut cranes = parse_cranes(&mut input);
    let mut cranes2 = cranes.clone();

    let instructions = input.map(|line| {
        line.split(|f| f.is_ascii_whitespace())
            .filter(|v| v[0].is_ascii_digit())
            .map(|v| to_str(v).parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap()
    });

    for (n, from, to) in instructions.clone() {
        for _ in 0..n {
            let v = cranes[from - 1].pop().unwrap();
            cranes[to - 1].push(v);
        }
    }
    let part1 = top_items(&cranes);
    assert!(part1 == b"BWNCQRMDB");
    println!("PART1: {}", to_str(&part1));

    for (n, from, to) in instructions.clone() {
        let len = &cranes2[from - 1].len();
        let removed = cranes2[from - 1].drain(len - n..).collect::<Vec<_>>();
        cranes2[to - 1].extend(removed);
    }

    let part2 = top_items(&cranes2);
    assert!(part2 == b"NHWZCBNBF");
    println!("PART2: {}", to_str(&part2));
}
