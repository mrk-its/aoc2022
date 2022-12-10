#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

pub mod display;

#[cfg(target_vendor = "atari8")]
#[path = "atari.rs"]
mod ui;

#[cfg(not(target_vendor = "atari8"))]
#[path = "sim.rs"]
mod ui;

use crate::display::DisplayInterface;

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
    align: usize,
    width: usize,
    offs: (i16, i16),
}
impl Plane {
    pub fn new(w: usize, h: usize, offs: (i16, i16)) -> Self {
        let width = w / 4;
        let size = width * h;
        let mut data = Vec::with_capacity(size + width);
        data.resize(size + width, 0);

        let align = 64 - (data.as_ptr() as usize & 63);

        Plane {
            data,
            width,
            offs,
            align,
        }
    }
    pub fn clean(&mut self) {
        self.data.fill(0);
    }
    pub fn mark(&mut self, pos: &(i16, i16)) -> (usize, usize) {
        let pos = (
            (pos.0 - self.offs.0) as usize,
            (pos.1 - self.offs.1) as usize,
        );
        let offs = self.align + pos.1 * self.width + pos.0 / 4;
        let bit_offs = (pos.0 & 3) * 2;
        let mask = 0x40 >> bit_offs;
        self.data[offs] |= mask;
        pos
    }
    pub fn mark_head(&mut self, pos: &(i16, i16), on: bool) -> (usize, usize) {
        let pos = (
            (pos.0 - self.offs.0) as usize,
            (pos.1 - self.offs.1) as usize,
        );
        let offs = self.align + pos.1 * self.width + pos.0 / 4;
        let bit_offs = (pos.0 & 3) * 2;
        let mask = 0x80 >> bit_offs;
        if on {
            self.data[offs] |= mask;
        } else {
            self.data[offs] &= !mask;
        }
        pos
    }
    pub fn count(&self) -> usize {
        self.data
            .iter()
            .flat_map(|b| (0..4).map(|v| (*b >> v * 2) & 1))
            .filter(|&v| v > 0)
            .count()
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

    let moves = utils::iter_lines!("../../input/day09/input.txt")
        .map(|line| (line[0], to_str(&line[2..]).parse::<u8>().unwrap()));

    let mut min = (-94, -12);
    let mut max = (141, 275);

    // println!("CHECKING SIZE...");
    // for (dir, n) in moves.clone() {
    //     rope.mv(dir, n);
    //     min.0 = min.0.min(rope.head.0);
    //     min.1 = min.1.min(rope.head.1);
    //     max.0 = max.0.max(rope.head.0);
    //     max.1 = max.1.max(rope.head.1);
    // }
    // rope.reset();

    let w = (max.0 - min.0 + 1 + 127) & !127;
    let h = max.1 - min.1 + 1;

    let mut plane = Plane::new(w as usize, h as usize, min);

    let mut display = ui::Display::init(
        w as usize,
        h as usize,
        plane.data[plane.align..].as_ptr() as *const u8,
    );

    plane.mark(&rope.tail);

    for (dir, n) in moves.clone() {
        for _ in 0..n {
            let pos = rope.step(dir);
            plane.mark_head(&rope.head, false);
            if rope.mv_to(&pos) {
                plane.mark_head(&rope.head, true);
                let abs_pos = plane.mark(&rope.tail);
                display.scroll_to(abs_pos.0, abs_pos.1);
            }
        }
    }
    display.hide();
    print!("PART1: ");
    let part1 = plane.count();
    assert!(part1 == 6057);
    println!("{}", part1);

    let mut ropes = [Rope::new(); 9];
    plane.clean();
    plane.mark(&ropes[0].tail);

    display.show();

    for (dir, n) in moves.clone() {
        for _ in 0..n {
            let mut pos = ropes[0].step(dir);
            let mut moved = true;
            for (n, rope) in ropes.iter_mut().enumerate() {
                if moved {
                    plane.mark_head(&rope.head, false);
                    moved = rope.mv_to(&pos);
                    plane.mark_head(&rope.head, true);
                    pos = rope.tail;
                    if n == 8 && moved {
                        let abs_pos = plane.mark(&rope.tail);
                        display.scroll_to(abs_pos.0, abs_pos.1);
                    }
                } else {
                    break;
                }
            }
        }
    }
    display.hide();
    print!("PART2: ");
    let part2 = plane.count();
    assert!(part2 == 2514);
    println!("{}", part2);
}
