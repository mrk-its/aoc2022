#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
#[allow(unused_imports)]
use mos_alloc;

utils::entry!(main);

use ufmt_stdio::*;
use utils::to_str;

type Int = i64;

struct Monkey {
    index: usize,
    items: Vec<Int>,
    op: Box<dyn Fn(Int) -> Int>,
    divider: Int,
    monkey_if_true: usize,
    monkey_if_false: usize,
    cnt: Int,
}

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
        builder.field("index", &self.index)?;
        builder.field("items", &self.items.as_slice())?;
        builder.field("divider", &self.divider)?;
        builder.field("monkey_if_true", &self.monkey_if_true)?;
        builder.field("monkey_if_false", &self.monkey_if_false)?;
        builder.field("cnt", &self.cnt)?;
        builder.finish()
    }
}

fn parse<'a>(mut input: impl Iterator<Item = &'a [u8]>) -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::with_capacity(10);
    // let mut input = utils::iter_lines!("../../input/day11/input.txt");

    loop {
        let index = match input.next() {
            Some(line) => to_str(&line[7..line.len() - 1]).parse::<usize>().unwrap(),
            None => break,
        };
        let items = to_str(&input.next().unwrap()[18..])
            .split(", ")
            .map(|f| f.parse().unwrap())
            .collect::<Vec<Int>>();

        let op_txt = &input.next().unwrap()[19..];
        let op: Box<dyn Fn(Int) -> Int> = if op_txt == b"old * old" {
            Box::new(move |old: Int| old * old)
        } else if op_txt.starts_with(b"old * ") {
            let arg: Int = to_str(&op_txt[6..]).parse().unwrap();
            Box::new(move |old: Int| old * arg)
        } else if op_txt.starts_with(b"old + ") {
            let arg: Int = to_str(&op_txt[6..]).parse().unwrap();
            Box::new(move |old: Int| old + arg)
        } else {
            unreachable!();
        };
        let test_txt = &input.next().unwrap()[8..];
        let divider = match &test_txt[0..3] {
            b"div" => to_str(&test_txt[13..]).parse::<Int>().unwrap(),
            _ => unreachable!(),
        };
        let if_true = &input.next().unwrap()[4..];
        let if_false = &input.next().unwrap()[4..];

        assert!(if_true.starts_with(b"If true:"));
        assert!(if_false.starts_with(b"If false:"));

        let monkey_if_true = to_str(&if_true[25..]).parse().unwrap();
        let monkey_if_false = to_str(&if_false[26..]).parse().unwrap();

        input.next();

        let monkey = Monkey {
            index,
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

fn play(mut monkeys: Vec<Monkey>, n_rounds: usize, divider: Int) -> Int {
    let dividers: Vec<_> = monkeys.iter().map(|m| m.divider).collect();
    let mult = dividers.iter().fold(1, |acc, val| acc * val);

    // let mut tmp = Vec::<Int>::with_capacity(100);

    for round in 0..n_rounds {
        for index in 0..monkeys.len() {
            for worry_level in &monkeys[index].items.drain(..).collect::<Vec<_>>() {
                monkeys[index].cnt += 1;
                let mut next_level = (monkeys[index].op)(*worry_level) % mult;
                if divider > 1 {
                    next_level /= divider;
                }
                let next_monkey = if next_level % monkeys[index].divider == 0 {
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
    let mut counts = monkeys.iter().map(|m| m.cnt).collect::<Vec<_>>();
    counts.sort_by_key(|i| -i);
    counts[0] * counts[1]
}

fn main() {
    // #[cfg(target_arch = "mos")]
    // mos_alloc::set_limit(20000);

    let iter = utils::iter_lines!("../../input/day11/input.txt");

    let part1 = play(parse(iter.clone()), 20, 3);
    assert_eq!(part1, 57348);
    println!("PART1: {}", part1);

    let part2 = play(parse(iter), 10000, 1);
    assert_eq!(part2, 14106266886);
    println!("PART2: {}", part2);
}
