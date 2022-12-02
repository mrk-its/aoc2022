#![no_std]
#![feature(start)]
utils::entry!(main);
use ufmt_stdio::*;

type Int = i32;

fn main() {
    let mut input = utils::iter_lines!("part1.txt");
    let mut maxes = [0; 3];
    loop {
        let sum = input
            .by_ref()
            .map_while(|line| utils::to_str(line).parse::<Int>().ok())
            .sum();
        if sum == 0 {
            break;
        }
        if sum > maxes[0] {
            maxes[0] = sum;
            maxes.sort_unstable();
        }
    }

    let part1 = *maxes.last().unwrap();
    let part2 = maxes.iter().sum::<Int>();

    assert_eq!(part1, 69626);
    assert_eq!(part2, 206780);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
