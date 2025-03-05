#![no_std]
#![no_main]

use ufmt_stdio::*;

const MAX_SIZE: usize = 99;
const MASK: u8 = 0x40;

struct Trees {
    trees: [u8; MAX_SIZE * MAX_SIZE],
    size: usize,
}

impl Trees {
    fn update_visibility_row(&mut self, x: usize, y: usize, dir: isize, mask: u8) {
        let mut offs = y * self.size + x;
        let mut max = 0;
        for i in 0..self.size {
            let v = &mut self.trees[offs];

            if i == 0 || (*v & 0xf) > max {
                max = *v & 0xf;
                *v |= mask
            }
            offs = offs.wrapping_add(dir as usize);
        }
    }

    fn update_visibility(&mut self) {
        for i in 0..self.size {
            self.update_visibility_row(0, i, 1, MASK);
            self.update_visibility_row(self.size - 1, i, -1, MASK);
            self.update_visibility_row(i, 0, self.size as isize, MASK);
            self.update_visibility_row(i, self.size - 1, -(self.size as isize), MASK);
        }
    }
    fn clear_visibility(&mut self) {
        for v in self.trees.iter_mut() {
            *v &= !MASK;
        }
    }

    fn look_at(&self, x: usize, y: usize, dir: isize, max_dist: usize) -> i8 {
        let mut offs = y * self.size + x;
        let mut dist = 0;
        let h = self.trees[offs];
        for _ in 0..max_dist {
            offs = offs.wrapping_add(dir as usize);
            let v = self.trees[offs];
            dist += 1;
            if v >= h {
                return dist;
            }
        }
        dist
    }
    fn look(&self) -> i32 {
        let mut score = 0;
        for y in 0..self.size {
            for x in 0..self.size {
                let a = self.look_at(x, y, -(self.size as isize), y) as i32;
                let b = self.look_at(x, y, -1, x) as i32;
                let c = self.look_at(x, y, 1, self.size - x - 1) as i32;
                let d = self.look_at(x, y, self.size as isize, self.size - y - 1) as i32;
                score = score.max(a * b * c * d);
            }
        }
        return score;
    }
}

#[cfg_attr(not(test), export_name = "main")]
#[cfg_attr(test, allow(dead_code))]
fn main() -> isize {
    let mut trees = Trees {
        trees: [0; MAX_SIZE * MAX_SIZE],
        size: 0,
    };

    let mut offs = 0;

    for line in utils::iter_lines!("../../input/day08/input.txt") {
        trees.trees[offs..offs + line.len()].copy_from_slice(line);
        offs += line.len();
        trees.size = line.len();
    }

    trees.update_visibility();

    let part1 = trees.trees.iter().filter(|v| **v >= MASK).count();
    assert!(part1 == 1832);
    println!("PART1: {}", part1);

    trees.clear_visibility();

    let part2 = trees.look();
    assert!(part2 == 157320);
    println!("PART2: {}", part2);
    return 0;
}
