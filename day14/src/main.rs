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

use ufmt_stdio::*;

include!(concat!(env!("OUT_DIR"), "/input.rs"));

fn get(memory: &mut [u8], p: &(i16, i16)) -> bool {
    let mask = 0x80 >> (p.0 & 7);
    (memory[p.1 as usize * 40 + p.0 as usize / 8] & mask) > 0
}

fn plot(memory: &mut [u8], p: &(i16, i16)) {
    let mask = 0x80 >> (p.0 & 7);
    memory[p.1 as usize * 40 + p.0 as usize / 8] |= mask;
}
fn clear(memory: &mut [u8], p: &(i16, i16)) {
    let mask = 0x80 >> (p.0 & 7);
    memory[p.1 as usize * 40 + p.0 as usize / 8] &= !mask;
}

fn draw_line(memory: &mut [u8], start: &(i16, i16), end: &(i16, i16)) {
    let mut start = *start;
    let dx = (end.0 - start.0).signum();
    let dy = (end.1 - start.1).signum();
    plot(memory, &start);
    while &start != end {
        start.0 += dx;
        start.1 += dy;
        plot(memory, &start);
    }
}

fn shift(pt: &(i16, i16)) -> (i16, i16) {
    return (160 + pt.0 - 500, pt.1);
}

fn main() {
    mos_alloc::set_limit(10000);
    let mut display = ui::Display::init();

    let memory = unsafe {
        let mem = display.get_screen_memory().add(0);
        core::slice::from_raw_parts_mut(mem, 200 * 40)
    };
    let mut max_y = 0;

    for &line in POINTS {
        let mut iter = line.iter().map(shift);
        let mut start = iter.next().unwrap();
        max_y = max_y.max(start.1);
        for end in iter {
            max_y = max_y.max(end.1);
            draw_line(memory, &start, &end);
            start = end;
            max_y = max_y.max(start.1).max(end.1)
        }
    }
    let mut count = 0;
    loop {
        display.clear_atract();
        let mut pt = shift(&(500, 0));
        if get(memory, &pt) {
            println!("PART2: {}", count);
            break;
        }
        loop {
            plot(memory, &pt);
            let mut moved = false;
            for dx in [0, -1, 1] {
                let dst = (pt.0 + dx, pt.1 + 1);
                if !get(memory, &dst) {
                    clear(memory, &pt);
                    pt = dst;
                    moved = true;
                    break;
                }
            }
            if !moved {
                count += 1;
                break;
            } else if pt.1 > max_y + 2 {
                println!("PART1: {}", count);
                draw_line(memory, &(0, max_y + 2), &(319, max_y + 2));
                break;
            }
        }
    }
}
