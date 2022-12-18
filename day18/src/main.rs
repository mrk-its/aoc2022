#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]
#![feature(slice_group_by)]

utils::entry!(main);
extern crate alloc;
use alloc::vec::Vec;
use itertools::Itertools;
#[allow(unused_imports)]
use mos_alloc;
use ufmt_stdio::*;
use utils::to_str;

const SIZE: usize = 20;

#[derive(Default)]
struct Layer {
    data: [[bool; SIZE]; SIZE],
}

impl Layer {
    fn get(&self, i: u8, j: u8) -> bool {
        self.data[i as usize][j as usize]
    }
    fn set(&mut self, i: u8, j: u8, v: bool) {
        self.data[i as usize][j as usize] = v;
    }
}

fn parse() -> Vec<(u8, u8, u8)> {
    let mut cubes: Vec<(u8, u8, u8)> = utils::iter_lines!("../../input/day18/input.txt")
        .map(|line| {
            line.split(|v| !v.is_ascii_digit())
                .filter(|v| v.len() > 0)
                .map(|v| to_str(v).parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    cubes.sort();
    cubes
}

fn part1(input: &Vec<(u8, u8, u8)>) {
    let total_surface = input.len() * 6;
    let mut removed_x = 0;
    let mut removed_y = 0;
    let mut removed_z = 0;

    let mut layer = Layer::default();

    for slice in input.group_by(|a, b| a.0 == b.0) {
        for &(_, y, z) in slice {
            if layer.get(y, z) {
                removed_x += 2;
            }
        }

        layer = Layer::default();
        for &(_, y, z) in slice {
            if y > 0 && layer.get(y - 1, z) {
                removed_y += 2;
            }
            if z > 0 && layer.get(y, z - 1) {
                removed_z += 2;
            }
            layer.set(y, z, true);
        }
    }
    println!(
        "total: {}, removed: {} {} {}",
        total_surface, removed_x, removed_y, removed_z
    );
    println!(
        "PART1: {}",
        total_surface - removed_x - removed_y - removed_z
    );
}

fn main() {
    #[cfg(target_arch = "mos")]
    mos_alloc::set_limit(20000);
    let data = parse();
    part1(&data);
}
