#![no_std]
#![feature(start)]

utils::entry!(main);
extern crate alloc;

use alloc::vec::Vec;
#[allow(unused_imports)]
use mos_alloc;
use ufmt_stdio::*;
use utils::to_str;

struct Shape {
    width: i8,
    data: &'static [u8],
}

impl Shape {
    fn data(&self, dx: i8) -> impl Iterator<Item = u8> + '_ {
        return self
            .data
            .iter()
            .rev()
            .map(move |v| v << ((8 - dx) - self.width));
    }
}

const SHAPES: &[Shape] = &[
    Shape {
        width: 4,
        data: &[0b1111],
    },
    Shape {
        width: 3,
        data: &[0b010, 0b111, 0b010],
    },
    Shape {
        width: 3,
        data: &[0b001, 0b001, 0b111],
    },
    Shape {
        width: 1,
        data: &[0b1, 0b1, 0b1, 0b1],
    },
    Shape {
        width: 2,
        data: &[0b11, 0b11],
    },
];

const PART2_N_CUBES: u64 = 1000000000000;
const WIN_LEN: usize = 20;

fn show_line(b: u8) {
    const chars: &[u8] = b" #";
    let mut buf = [0u8; 8];
    let mut mask = 0x80;
    for index in 0..8u8 {
        buf[index as usize] = chars[(b & mask > 0) as usize];
        mask >>= 1;
    }
    println!("{}", to_str(&buf));
}

fn show_chamber(chamber: &Vec<u8>) {
    for v in chamber.iter().rev() {
        show_line(*v);
    }
    println!();
}

fn main() {
    #[cfg(target_arch = "mos")]
    mos_alloc::set_limit(20000);

    // investigate why iter.cycle() doesn't work with debug
    // in release mode it stops working after turning off `panic_immediate_abort`

    let mut jets = utils::iter_lines!("../../input/day17/test.txt")
        .next()
        .unwrap()
        .iter()
        .cycle()
        .map(|&v| v as i8 - 61);

    let shapes = SHAPES.iter().cycle();

    let mut chamber: Vec<u8> = Vec::new();
    let mut part1 = 0;
    let mut part2 = 0;

    let mut rows_period = 0;
    let mut shapes_period = 0;
    let mut step2_n_row = 0;
    let mut step3_n_shape = 0;

    for (n, shape) in shapes.enumerate() {
        if n == 2022 {
            part1 = chamber.len();
            let end_index = chamber.len() - WIN_LEN;

            let slice = &chamber[end_index - WIN_LEN..end_index];
            assert!(slice.len() == WIN_LEN);

            for i in (0..end_index - WIN_LEN).rev() {
                if &chamber[i..i + slice.len()] == slice {
                    rows_period = end_index - WIN_LEN - i;
                    step2_n_row = chamber.len() + rows_period;
                    break;
                }
            }
        } else if step2_n_row > 0 && chamber.len() == step2_n_row {
            shapes_period = n - 2022;
            let rem = ((PART2_N_CUBES - n as u64) % shapes_period as u64) as usize;
            step3_n_shape = n + rem;
        } else if step3_n_shape > 0 && n == step3_n_shape {
            let remaining_cubes = PART2_N_CUBES - n as u64;
            assert!(remaining_cubes % shapes_period as u64 == 0);
            let remaining_height = (remaining_cubes / shapes_period as u64) * rows_period as u64;
            part2 = remaining_height + chamber.len() as u64;
            break;
        }

        let shape_h = shape.data.len();
        chamber.extend((0..shape_h + 3).map(|_| 0));
        let mut dx = 2 + jets.next().unwrap();

        let mut dst_y = chamber.len() - shape_h - 1;
        loop {
            let dst = &chamber[dst_y..dst_y + shape_h];
            assert!(dst.len() == shape_h);
            let fits = shape.data(dx).zip(dst).all(|v| v.0 & v.1 == 0);
            if !fits {
                dst_y = dst_y + 1;
                break;
            }
            let ndx = (dx + jets.next().unwrap()).max(0).min(7 - shape.width);
            let fits_jetted = shape.data(ndx).zip(dst).all(|v| v.0 & v.1 == 0);
            if fits_jetted {
                dx = ndx;
            }
            if dst_y == 0 {
                break;
            }
            dst_y = dst_y - 1;
        }
        chamber[dst_y..dst_y + shape_h]
            .iter_mut()
            .zip(shape.data(dx))
            .for_each(|(a, b)| *a |= b);

        while chamber.last() == Some(&0) {
            chamber.pop();
        }
    }
    println!("CHAMBER SIZE: {}", chamber.len());

    println!("PART1: {}", part1);
    println!("PART2: {}", part2);
}
