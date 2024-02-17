#![no_std]
#![feature(start)]

utils::entry!(main);
extern crate alloc;

use alloc::vec::Vec;
use itertools::Itertools;
#[allow(unused_imports)]
use mos_alloc;
use ufmt_stdio::{ufmt::derive::uDebug, *};
use utils::to_str;

#[derive(uDebug)]
struct Sensor {
    sensor: (i32, i32),
    becon: (i32, i32),
    dist: i32,
}
fn manhatan_dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
}

fn perimeter(mid: (i32, i32), r: i32) -> impl Iterator<Item = (i32, i32)> {
    (0..r).flat_map(move |v| {
        [
            (mid.0 + v, mid.1 - r + v),
            (mid.0 + r - v, mid.1 + v),
            (mid.0 - v, mid.0 + r - v),
            (mid.0 - r + v, mid.1 - v),
        ]
    })
}

fn main() {
    const PART1_ROW: i32 = 2000000;
    const PART2_SIZE: i32 = 4000000;

    //const PART1_ROW: i32 = 10;
    //const PART2_SIZE: i32 = 20;

    let sensors = utils::iter_lines!("../../input/day15/input.txt")
        .map(|line| {
            line.split(|c| !c.is_ascii_digit() && *c != b'-')
                .filter(|v| v.len() > 0)
                .map(|v| to_str(v).parse::<i32>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .map(|(a, b, c, d)| Sensor {
                    sensor: (a, b),
                    becon: (c, d),
                    dist: manhatan_dist((a, b), (c, d)),
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_dist = i32::MIN;

    for sensor in &sensors {
        min_x = min_x.min(sensor.sensor.0.min(sensor.becon.0));
        min_y = min_y.min(sensor.sensor.1.min(sensor.becon.1));
        max_x = max_x.max(sensor.sensor.0.max(sensor.becon.0));
        max_y = max_y.max(sensor.sensor.1.max(sensor.becon.1));
        max_dist = max_dist.max(sensor.dist);
    }

    let part1 = (-min_x - max_dist..max_x + max_dist)
        .map(|x| (x, PART1_ROW))
        .map(|pos| {
            sensors
                .iter()
                .map(|s| manhatan_dist(pos, s.sensor) <= s.dist && pos != s.becon)
                .any(|f| f) as i32
        })
        .sum::<i32>();
    assert!(part1 == 5100463);
    println!("PART1: {}", part1);

    let out = sensors
        .iter()
        .flat_map(|s| perimeter(s.sensor, s.dist + 1))
        .filter(|v| v.0 >= 0 && v.0 <= PART2_SIZE && v.1 >= 0 && v.1 <= PART2_SIZE)
        .filter(|pos| {
            sensors
                .iter()
                .map(|s| manhatan_dist(*pos, s.sensor) > s.dist)
                .all(|f| f)
        })
        .next()
        .unwrap();

    let part2 = out.0 as i64 * 4000000 + out.1 as i64;
    assert!(part2 == 11557863040754);
    println!("PART2: {:?}", part2);
}
