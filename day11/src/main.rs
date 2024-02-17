#![no_std]
#![feature(start)]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
#[allow(unused_imports)]
use mos_alloc;

#[cfg(target_vendor = "atari8")]
use a800xl_utils::clock;

utils::entry!(main);

use ufmt_stdio::*;
use utils::to_str;

type Int = i32;
type BigInt = i64;

struct Monkey {
    items: Vec<Int>,
    op: Box<dyn Fn(BigInt) -> BigInt>,
    divider: u8,
    monkey_if_true: usize,
    monkey_if_false: usize,
    cnt: Int,
}

// Hack for no symbol found during linking
#[no_mangle]
extern "C" fn __multi3(_a: i64, _b: i64) -> i64 {
    panic!();
}

impl uDebug for Monkey {
    fn fmt<W>(&self, formatter: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut builder = formatter.debug_struct("Monkey")?;
        builder.field("items", &self.items.as_slice())?;
        builder.field("divider", &self.divider)?;
        builder.field("monkey_if_true", &self.monkey_if_true)?;
        builder.field("monkey_if_false", &self.monkey_if_false)?;
        builder.field("cnt", &self.cnt)?;
        builder.finish()
    }
}

#[inline(never)]
fn parse_int(txt: &[u8]) -> Int {
    to_str(txt).parse::<u8>().unwrap() as Int
}
#[inline(never)]
fn next_line<'a>(iter: &mut impl Iterator<Item = &'a [u8]>, index: usize) -> &'a [u8] {
    &iter.next().unwrap()[index..]
}
fn parse<'a>(mut input: impl Iterator<Item = &'a [u8]>) -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::with_capacity(10);

    loop {
        if input.next().is_none() {
            break;
        }
        let items = next_line(&mut input, 18)
            .split(|&c| c == b',' || c == b' ')
            .filter(|v| v.len() > 0)
            .map(parse_int)
            .collect::<Vec<Int>>();

        let op_txt = next_line(&mut input, 23);
        let op: Box<dyn Fn(BigInt) -> BigInt> = if op_txt == b"* old" {
            Box::new(move |old: BigInt| old * old)
        } else if op_txt[0] == b'*' {
            let arg = parse_int(&op_txt[2..]) as BigInt;
            Box::new(move |old: BigInt| old * arg)
        } else if op_txt[0] == b'+' {
            let arg = parse_int(&op_txt[2..]) as BigInt;
            Box::new(move |old: BigInt| old + arg)
        } else {
            unreachable!();
        };
        let divider = parse_int(next_line(&mut input, 21)) as u8;
        let monkey_if_true = parse_int(next_line(&mut input, 29)) as usize;
        let monkey_if_false = parse_int(next_line(&mut input, 30)) as usize;

        input.next();

        let monkey = Monkey {
            items,
            op,
            divider,
            monkey_if_false,
            monkey_if_true,
            cnt: 0,
        };
        monkeys.push(monkey);
    }
    monkeys
}

fn play(mut monkeys: Vec<Monkey>, n_rounds: usize, divider: u8) -> BigInt {
    let dividers: Vec<_> = monkeys.iter().map(|m| m.divider as i32).collect();
    let mult = dividers.iter().fold(1, |acc, val| acc * val) as BigInt;

    let mut tmp = Vec::<Int>::with_capacity(40);

    for round in 0..n_rounds {
        for index in 0..monkeys.len() {
            tmp.clear();
            tmp.extend(monkeys[index].items.drain(..));
            for worry_level in &tmp {
                monkeys[index].cnt += 1;
                let mut next_level = ((monkeys[index].op)(*worry_level as BigInt) % mult) as Int;
                if divider > 1 {
                    next_level /= divider as Int;
                }
                let next_monkey = if next_level % monkeys[index].divider as Int == 0 {
                    monkeys[index].monkey_if_true
                } else {
                    monkeys[index].monkey_if_false
                };
                monkeys[next_monkey].items.push(next_level);
            }
        }
        if round > 0 && (round & 255) == 0 {
            println!("round #{}", round);
        }
    }
    let mut counts = monkeys.iter().map(|m| m.cnt as i32).collect::<Vec<_>>();
    counts.sort_by_key(|i| -i);
    counts[0] as BigInt * counts[1] as BigInt
}

fn main() {
    #[cfg(target_vendor = "atari8")]
    let start_t = clock();

    let iter = utils::iter_lines!("../../input/day11/input.txt");

    let part1 = play(parse(iter.clone()), 20, 3);
    assert_eq!(part1, 57348);
    println!("PART1: {}", part1);

    let part2 = play(parse(iter), 10000, 1);
    assert_eq!(part2, 14106266886);
    println!("PART2: {}", part2);

    #[cfg(target_vendor = "atari8")]
    {
        let ticks = clock() - start_t;
        let seconds = ticks / 50;
        println!("ticks: {} ({} seconds)", ticks, seconds);
    }
}
