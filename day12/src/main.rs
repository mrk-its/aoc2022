#![no_std]
#![feature(start)]

utils::entry!(main);
use ufmt_stdio::*;
extern crate alloc;
use alloc::vec::Vec;

fn main() {
    mos_alloc::set_limit(16000);

    let height_map = include_bytes!("../../input/day12/input.txt");
    let width: isize = height_map.iter().position(|&c| c == b'\n').unwrap() as isize + 1;
    let offsets: [isize; 4] = [-width, -1, 1, width];
    let start = height_map.iter().position(|&v| v == b'E').unwrap();

    let height = |v| match v {
        b'S' => b'a',
        b'E' => b'z',
        _ => v,
    } as i8;

    let neighbours = |index| {
        let h = height(height_map[index]);
        offsets
            .iter()
            .cloned()
            .map(move |v| v + index as isize)
            .filter(move |&v| v >= 0 && v < height_map.len() as isize)
            .filter(move |&v| h <= height(height_map[v as usize]) + 1)
            .map(|v| v as usize)
    };

    let mut data: Vec<u16> = Vec::with_capacity(height_map.len());
    data.resize(height_map.len(), 0x7fff);

    let mut part2 = 0x7fff;

    let visited = |v: u16| v >= 0x8000;

    data[start] = 0;
    let part1 = 'outer: loop {
        let index = data
            .iter()
            .cloned()
            .enumerate()
            .filter(|&(_, v)| !visited(v))
            .min_by_key(|&v| v.1)
            .unwrap()
            .0;

        if height_map[index] == b'a' {
            part2 = part2.min(data[index]);
        } else if height_map[index] == b'S' {
            break 'outer data[index];
        }
        assert!(!visited(data[index]));
        let alt = data[index] + 1;
        data[index] |= 0x8000;
        for neigh in neighbours(index)
            .filter(|&v| !visited(data[v]))
            .collect::<Vec<_>>()
        {
            if alt < data[neigh] {
                data[neigh] = alt;
            }
        }
    };

    // assert_eq!(part1, 31);
    println!("PART1: {}", part1);
    // assert_eq!(part2, 0);
    println!("PART2: {}", part2);
}
