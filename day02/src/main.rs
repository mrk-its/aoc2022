#![no_std]
#![feature(start)]
utils::entry!(main);
use ufmt_stdio::*;

type Score = i16;
type Pair = u8;

const SCORES: [u8; 11] = [3, 6, 0, 0, 0, 3, 6, 0, 6, 0, 3];
const THINGS: [u8; 11] = [2, 0, 1, 0, 0, 1, 2, 0, 1, 2, 0];

fn parse_line(line: &[u8]) -> Pair {
    return (line[0] - b'A') * 4 + line[2] - b'X'
}

fn score(pair: Pair) -> Score {
    (SCORES[pair as usize] + 1 + (pair & 3)) as Score
}

fn score2(pair: Pair) -> Score {
    score((pair & !3) + THINGS[pair as usize])
}

fn main() {
    let input = utils::iter_lines!("input.txt").map(parse_line);
    let part1 = input.clone().map(score).sum::<Score>();
    let part2 = input.map(score2).sum::<Score>();

    assert!(part1 == 13268);
    assert!(part2 == 15508);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
