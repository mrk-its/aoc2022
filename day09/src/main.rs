#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

utils::entry!(main);
extern crate alloc;

use alloc::vec::Vec;
use ufmt_stdio::*;
use utils::to_str;

#[derive(Copy, Clone)]
struct Rope {
    head: (i16, i16),
    tail: (i16, i16),
}
struct Plane {
    data: Vec<u8>,
    width: usize,
    offs: (i16, i16),
}
impl Plane {
    pub fn new(w: usize, h: usize, offs: (i16, i16)) -> Self {
        let width = (w + 7) / 8;
        let size = width * h;
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        Plane { data, width, offs }
    }
    pub fn clean(&mut self) {
        self.data.fill(0);
    }
    pub fn mark(&mut self, pos: &(i16, i16)) {
        let pos = (
            (pos.0 - self.offs.0) as usize,
            (pos.1 - self.offs.1) as usize,
        );
        let offs = pos.1 * self.width + pos.0 / 8;
        let bit_offs = pos.0 & 7;
        let mask = 0x80 >> bit_offs;
        self.data[offs] |= mask;
    }
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            head: (0, 0),
            tail: (0, 0),
        }
    }
    pub fn reset(&mut self) {
        self.head = (0, 0);
        self.tail = (0, 0);
    }
    pub fn mv(&mut self, dir: u8, n: u8) {
        for _ in 0..n {
            let pos = self.step(dir);
            self.mv_to(&pos);
        }
    }
    pub fn step(&mut self, dir: u8) -> (i16, i16) {
        match dir {
            b'L' => (self.head.0 - 1, self.head.1),
            b'R' => (self.head.0 + 1, self.head.1),
            b'U' => (self.head.0, self.head.1 + 1),
            b'D' => (self.head.0, self.head.1 - 1),
            _ => unreachable!(),
        }
    }
    pub fn mv_to(&mut self, pos: &(i16, i16)) -> bool {
        self.head = *pos;
        let dx = self.head.0 - self.tail.0;
        let dy = self.head.1 - self.tail.1;
        let moved = dx.abs() > 1 || dy.abs() > 1;
        if moved {
            self.tail.0 += dx.signum();
            self.tail.1 += dy.signum();
        }
        moved
        // println!(
        //     "head: {} {}, tail: {} {}",
        //     self.head.0, self.head.1, self.tail.0, self.tail.1
        // );
    }
}

fn main() {
    let mut rope = Rope::new();
    mos_alloc::set_limit(20000);

    let moves = utils::iter_lines!("input.txt")
        .map(|line| (line[0], to_str(&line[2..]).parse::<u8>().unwrap()));

    let mut min = (0, 0);
    let mut max = (0, 0);

    for (dir, n) in moves.clone() {
        rope.mv(dir, n);
        min.0 = min.0.min(rope.head.0);
        min.1 = min.1.min(rope.head.1);
        max.0 = max.0.max(rope.head.0);
        max.1 = max.1.max(rope.head.1);
    }
    rope.reset();

    let w = max.0 - min.0 + 1;
    let h = max.1 - min.1 + 1;

    let mut plane = Plane::new(w as usize, h as usize, min);
    plane.mark(&rope.tail);

    for (dir, n) in moves.clone() {
        for _ in 0..n {
            let pos = rope.step(dir);
            if rope.mv_to(&pos) {
                // println!("tail: {} {}", tail_pos.0, tail_pos.1);
                plane.mark(&rope.tail);
            }
        }
    }

    let part1 = plane
        .data
        .iter()
        .flat_map(|b| (0..8).map(|v| (*b >> v) & 1))
        .filter(|&v| v > 0)
        .count();

    let mut ropes = [Rope::new(); 9];
    plane.clean();
    plane.mark(&ropes[0].tail);

    assert!(part1 == 6057);
    println!("PART1: {}", part1);

    for (dir, n) in moves.clone() {
        for _ in 0..n {
            let mut pos = ropes[0].step(dir);
            let mut moved = true;
            for (n, rope) in ropes.iter_mut().enumerate() {
                if moved {
                    moved = rope.mv_to(&pos);
                    pos = rope.tail;
                    if n == 8 && moved {
                        plane.mark(&rope.tail);
                    }
                } else {
                    break;
                }
            }
        }
    }

    let part2 = plane
        .data
        .iter()
        .flat_map(|b| (0..8).map(|v| (*b >> v) & 1))
        .filter(|&v| v > 0)
        .count();
    assert!(part2 == 2514);
    println!("PART2: {}", part2);
}
