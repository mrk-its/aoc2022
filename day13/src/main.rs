#![no_std]
#![feature(start)]

extern crate alloc;
extern crate mos_alloc;

utils::entry!(main);
use core::{cell::RefCell, cmp::Ordering};

use alloc::vec;
use alloc::vec::Vec;
use itertools::{put_back, PutBack};
use ufmt_stdio::*;

#[derive(Eq, PartialEq, Clone)]
enum Item {
    Number(u8),
    List(Vec<Item>),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a.partial_cmp(b),
            (Item::List(a), Item::List(b)) => a.partial_cmp(b),
            (Item::List(_), Item::Number(_)) => self.partial_cmp(&other.to_list()),
            (Item::Number(_), Item::List(_)) => self.to_list().partial_cmp(other),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a.cmp(b),
            (Item::List(a), Item::List(b)) => a.cmp(b),
            (Item::List(_), Item::Number(_)) => self.cmp(&other.to_list()),
            (Item::Number(_), Item::List(_)) => self.to_list().cmp(other),
        }
    }
}

impl Item {
    fn is_num(&self) -> bool {
        match &self {
            Item::Number(_) => true,
            _ => false,
        }
    }
    fn to_list(&self) -> Item {
        match &self {
            Item::Number(n) => Item::List(vec![Item::Number(*n)]),
            _ => unreachable!(),
        }
    }
    fn _parse(input: &[u8]) -> Option<Item> {
        return Item::parse(&mut put_back(input.iter().cloned()));
    }
    fn parse<I>(input: &mut PutBack<I>) -> Option<Item>
    where
        I: Iterator<Item = u8>,
    {
        let c = input.next().unwrap();
        match c {
            b'0'..=b'9' => {
                let mut num = c - b'0';
                match input.next() {
                    Some(c) => {
                        if c.is_ascii_digit() {
                            num = num * 10 + c - b'0';
                        } else {
                            input.put_back(c);
                        }
                    }
                    _ => {}
                }
                Some(Item::Number(num))
            }
            b'[' => {
                let mut list = Vec::new();
                while let Some(item) = Item::parse(input) {
                    list.push(item);
                    match input.next() {
                        Some(b',') => (),
                        Some(b']') => break,
                        _ => unreachable!(),
                    }
                }
                // list.shrink_to_fit();
                Some(Item::List(list))
            }
            _ => None,
        }
    }
}

fn main() {
    mos_alloc::set_limit(10000);
    let ignore_below: RefCell<Option<Item>> = core::cell::RefCell::new(None);

    let input = utils::iter_lines!("../../input/day13/test.txt")
        .filter(|line| line.len() > 0)
        .map(|line| Item::_parse(line).unwrap());

    let mut part1 = 0;
    let mut iter = input.clone();
    loop {
        let a = match iter.next() {
            Some(a) => a,
            _ => break,
        };
        let b = iter.next().unwrap();
        if a <= b {
            part1 += 1;
        }
    }

    println!("PART1: {}", part1);

    let markers = &[
        Item::_parse(b"[[2]]").unwrap(),
        Item::_parse(b"[[6]]").unwrap(),
    ];

    let items = markers.iter().cloned().chain(input.clone());

    let cnt = items.clone().count();

    let mut weights = Vec::with_capacity(cnt);
    weights.resize(cnt, 0);

    let mut current_weight = 0;
    loop {
        let min = items
            .clone()
            .enumerate()
            .filter(|item| match &*ignore_below.borrow() {
                Some(ignore_below) => &item.1 > &ignore_below,
                _ => true,
            })
            .min_by_key(|f| f.1.clone());

        let min = match min {
            Some(i) => i,
            None => break,
        };
        weights[min.0] = current_weight;
        current_weight += 1;
        ignore_below.replace(Some(min.1));
    }
    let part2 = (weights[0] + 1) * (weights[1] + 1);
    println!("PART2: {}", part2);
    return;
}
