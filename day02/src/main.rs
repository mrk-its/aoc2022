#![no_std]
#![feature(start)]
utils::entry!(main);
use ufmt_stdio::*;

type Score = i32;

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

const LOSE: u8 = 1;
const DRAW: u8 = 2;
const WIN: u8 = 3;

fn parse(code: u8) -> u8 {
    return if code <= b'C' {
        code - b'A' + 1
    } else {
        code - b'X' + 1
    }
}

fn parse_line(line: &[u8]) -> (u8, u8) {
    return (parse(line[0]), parse(line[2]))
}


fn score(pair: (u8, u8)) -> Score {
    let outcome = match pair {
        (ROCK, PAPER) => 6,
        (ROCK, SCISSORS) => 0,
        (PAPER, ROCK) => 0,
        (PAPER, SCISSORS) => 6,
        (SCISSORS, ROCK) => 6,
        (SCISSORS, PAPER) => 0,
        _ => 3,
    };
    pair.1 as Score + outcome
}

fn determine_thing(thing: u8, result: u8) -> u8 {
    match (thing, result) {
        (ROCK, LOSE) => SCISSORS,
        (ROCK, DRAW) => ROCK,
        (ROCK, WIN) => PAPER,
        (PAPER, LOSE) => ROCK,
        (PAPER, DRAW) => PAPER,
        (PAPER, WIN) => SCISSORS,
        (SCISSORS, LOSE) => PAPER,
        (SCISSORS, DRAW) => SCISSORS,
        (SCISSORS, WIN) => ROCK,
        _ => panic!(),
    }
}

fn score2(pair: (u8, u8)) -> Score {
    let thing = determine_thing(pair.0, pair.1);
    score((pair.0, thing))
}

fn main() {
    let input = utils::iter_lines!("input.txt").map(parse_line);
    let part1 = input.clone().map(score).sum::<Score>();
    let part2 = input.map(score2).sum::<Score>();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
