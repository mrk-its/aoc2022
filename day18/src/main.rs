#![no_std]
#![feature(start)]
#![allow(dead_code)]

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
    fn get(&self, i: i8, j: i8) -> bool {
        self.data[i as usize][j as usize]
    }
    fn set(&mut self, i: i8, j: i8, v: bool) {
        self.data[i as usize][j as usize] = v;
    }
    fn show(&self) {
        for row in &self.data {
            for &c in row {
                if c {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

const R: u8 = 1;
const L: u8 = 2;
const U: u8 = 4;
const D: u8 = 8;
const B: u8 = 16;
const F: u8 = 32;
const OCCUPIED: u8 = 64;
const VISITED: u8 = 128;
const GRAY: u8 = 1;

#[derive(Default)]
struct Volume<const N: usize> {
    data: [[[u8; SIZE + 2]; SIZE + 2]; SIZE + 2],
}

impl<const N: usize> Volume<N> {
    fn set(&mut self, x: i8, y: i8, z: i8, v: u8) {
        self.data[x as usize][y as usize][z as usize] = v;
    }
    fn get(&self, x: i8, y: i8, z: i8) -> u8 {
        self.data[x as usize][y as usize][z as usize]
    }
    fn get_mut(&mut self, x: i8, y: i8, z: i8) -> &mut u8 {
        &mut self.data[x as usize][y as usize][z as usize]
    }
    fn update_face(&mut self, x: i8, y: i8, z: i8, dir: u8) {
        const S: i8 = SIZE as i8 + 2;
        if x >= 0 && y >= 0 && z >= 0 && x < S && y < S && z < S {
            let v = self.get_mut(x, y, z);
            if *v & OCCUPIED > 0 {
                *v |= dir;
            }
        }
    }
    fn neighbours(&self, x: i8, y: i8, z: i8, out: &mut Vec<(i8, i8, i8)>) {
        const S: i8 = SIZE as i8 + 2;
        out.clear();
        out.extend(
            [
                (-1, 0, 0),
                (1, 0, 0),
                (0, -1, 0),
                (0, 1, 0),
                (0, 0, -1),
                (0, 0, 1),
            ]
            .iter()
            .map(move |&(dx, dy, dz)| (x + dx, y + dy, z + dz))
            .filter(|&(x, y, z)| x >= 0 && y >= 0 && z >= 0 && x < S && y < S && z < S)
            .filter(|&(x, y, z)| self.get(x, y, z) & OCCUPIED == 0),
        )
    }
    fn bfs(&mut self, x: i8, y: i8, z: i8) {
        let mut max_size = 0;
        let mut to_visit: Vec<(i8, i8, i8)> = Vec::with_capacity(N);
        let mut neighbours = Vec::with_capacity(6);
        to_visit.push((x, y, z));
        self.set(x, y, z, GRAY);

        while let Some((x, y, z)) = to_visit.pop() {
            // println!("q size: {}", to_visit.len());
            // #[cfg(target_arch = "mos")]
            // println!("free: {}", mos_alloc::bytes_free());
            self.update_face(x - 1, y, z, R);
            self.update_face(x + 1, y, z, L);
            self.update_face(x, y - 1, z, U);
            self.update_face(x, y + 1, z, D);
            self.update_face(x, y, z - 1, B);
            self.update_face(x, y, z + 1, F);

            self.neighbours(x, y, z, &mut neighbours);

            for &(x, y, z) in &neighbours {
                let v = self.get_mut(x, y, z);
                if *v & (VISITED | GRAY) == 0 {
                    *v = GRAY;
                    to_visit.push((x, y, z))
                }
            }
            max_size = max_size.max(to_visit.len());
            *self.get_mut(x, y, z) |= VISITED;
        }
        println!(
            "max q size: {}, q capacity: {}",
            max_size,
            to_visit.capacity()
        );
    }
    fn show(&self, mask: u8) {
        for plane in &self.data {
            for row in plane {
                for &c in row {
                    if c & mask > 0 {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
        println!();
    }
}

include!(concat!(env!("OUT_DIR"), "/input.rs"));
fn parse() -> &'static [(i8, i8, i8)] {
    POINTS
}

fn parse2() -> Vec<(i8, i8, i8)> {
    let mut cubes: Vec<(i8, i8, i8)> = utils::iter_lines!("../../input/day18/input.txt")
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

fn part1(input: &[(i8, i8, i8)]) {
    let total_surface = input.len() * 6;
    let mut removed_x = 0;
    let mut removed_y = 0;
    let mut removed_z = 0;

    let mut layer = Layer::default();

    for slice in input[..].chunk_by(|a, b| a.0 == b.0) {
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
        layer.show();
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

fn part2(input: &[(i8, i8, i8)]) {
    // llvm-mos BUG!
    // after changing capacity to 5000 program enter infinite loop
    // on sim it occurs only if whole input data is embedded (with parse2)
    // on atari8 it fails with this configuration unless part1 is commented out

    let mut volume = Volume::<4000>::default();
    for &(x, y, z) in input {
        volume.set(x + 1, y + 1, z + 1, OCCUPIED);
    }
    volume.bfs(0, 0, 0);
    // volume.show(VISITED);
    let mut face_cntrs = [0, 0, 0, 0, 0, 0];
    for plane in &volume.data {
        for row in plane {
            for v in row {
                for (mask, cnt) in [L, R, U, D, F, B].iter().zip(face_cntrs.iter_mut()) {
                    if (v & OCCUPIED > 0) && (v & *mask) > 0 {
                        *cnt += 1;
                    }
                }
            }
        }
    }
    println!("PART2: {:?} {}", face_cntrs, face_cntrs.iter().sum::<i32>());
}

fn main() {
    #[cfg(target_arch = "mos")]
    mos_alloc::set_limit(20000);
    let data = parse();
    #[cfg(target_arch = "mos")]
    println!("free: {}", mos_alloc::bytes_free());

    part1(&data);
    part2(&data);
}
