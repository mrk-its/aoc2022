#![no_std]
#![feature(start)]

utils::entry!(main);
use ufmt_stdio::{ufmt::derive::uDebug, *};
use utils::to_str;

type Int = i16;

#[derive(uDebug, Clone, Copy)]
enum Instr {
    AddX(Int),
    Noop,
}

#[derive(uDebug)]
struct CPU {
    current: Instr,
    instr_cycles: usize,
    x: Int,
    cycles: usize,
}

impl CPU {
    pub fn execute(&mut self, instr: &Instr) {
        match &instr {
            Instr::Noop => (),
            Instr::AddX(n) => {
                self.x += n;
            }
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            x: 1,
            cycles: 0,
            instr_cycles: 0,
            current: Instr::Noop,
        }
    }
}

#[cfg(not(any(target_vendor = "atari8", target_vendor = "sim")))]
fn show_pixel(is_lit: bool) {
    println!("{}", if is_lit { "#" } else { "." });
}
#[cfg(target_vendor = "atari8")]
fn show_pixel(is_lit: bool) {
    print!("{}", to_str(if is_lit { b"\xa0" } else { b" " }));
}
#[cfg(target_vendor = "sim")]
fn show_pixel(is_lit: bool) {
    print!("{}", if is_lit { "█" } else { " " });
}

#[cfg(target_vendor = "atari8")]
fn eol() {}

#[cfg(not(target_vendor = "atari8"))]
fn eol() {
    println!();
}

fn main() {
    #[cfg(target_vendor = "atari8")]
    unsafe {
        *a800xl_utils::consts::LMARGN = 0;
        println!("");
    }

    let mut cpu = CPU::default();
    let parser = utils::iter_lines!("../../input/day10/input.txt")
        .map(|line| match &line[0..4] {
            b"addx" => (Instr::AddX(to_str(&line[5..]).parse().unwrap()), 2),
            b"noop" => (Instr::Noop, 1),
            _ => unreachable!(),
        })
        .flat_map(|(instr, cycle)| (0..cycle).rev().map(move |cycle| (instr, cycle)));

    let mut part1 = 0;

    let mut col = 0;

    for (instr, cycle) in parser {
        cpu.cycles += 1;
        show_pixel(col >= cpu.x - 1 && col <= cpu.x + 1);
        col += 1;
        if col == 40 {
            col = 0;
            eol();
        }
        if [20, 60, 100, 140, 180, 220].contains(&cpu.cycles) {
            part1 += cpu.x as i32 * cpu.cycles as i32;
        }

        if cycle == 0 {
            cpu.execute(&instr);
        }
    }
    println!("\nPART1: {}", part1);
}
