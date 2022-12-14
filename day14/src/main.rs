#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

pub mod display;
mod point;
use point::Point;

#[cfg(target_vendor = "atari8")]
#[path = "atari.rs"]
mod ui;

#[cfg(not(target_vendor = "atari8"))]
#[path = "sim.rs"]
mod ui;

use crate::display::DisplayInterface;

utils::entry!(main);
extern crate alloc;

use ufmt_stdio::{ufmt::derive::uDebug, *};

include!(concat!(env!("OUT_DIR"), "/input.rs"));

fn shift(pt: &(i16, i16)) -> (i16, i16) {
    return (160 + pt.0 - 500, pt.1);
}

fn main() {
    mos_alloc::set_limit(10000);
    let mut display = ui::Display::init();

    let memory = display.get_screen_memory();
    let mut max_y = 0;

    for &line in POINTS {
        let mut iter = line.iter().map(shift);
        let start = iter.next().unwrap();
        let mut pt = Point::from_coords(memory, &start);
        max_y = max_y.max(start.1);
        for end in iter {
            let dst = Point::from_coords(memory, &end);
            max_y = max_y.max(end.1);
            pt.line_to(&dst);
            pt = dst;
            max_y = max_y.max(end.1);
        }
    }

    let mut count = 0;
    let max_y_pt = Point::from_coords(memory, &(0, max_y + 2));
    loop {
        display.clear_atract();
        let mut pt = Point::from_coords(memory, &shift(&(500, 0)));
        if pt.get() {
            println!("PART2: {}", count);
            break;
        }
        loop {
            pt.set();
            let mut moved = false;
            for dx in [0, -1, 1] {
                let dst = pt.relative(dx, 1);
                if !dst.get() {
                    pt.clear();
                    pt = dst;
                    moved = true;
                    break;
                }
            }
            if !moved {
                count += 1;
                break;
            } else if pt >= max_y_pt {
                println!("PART1: {}", count);
                Point::from_coords(memory, &(0, max_y + 2))
                    .line_to(&Point::from_coords(memory, &(319, max_y + 2)));
                break;
            }
        }
    }
}
