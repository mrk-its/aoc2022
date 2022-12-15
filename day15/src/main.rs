#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

utils::entry!(main);
extern crate alloc;

use alloc::vec::Vec;
use itertools::Itertools;
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

fn main() {
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

    for sensor in &sensors {
        min_x = min_x.min(sensor.sensor.0.min(sensor.becon.0));
        min_y = min_y.min(sensor.sensor.1.min(sensor.becon.1));
        max_x = max_x.max(sensor.sensor.0.max(sensor.becon.0));
        max_y = max_y.max(sensor.sensor.1.max(sensor.becon.1));
    }

    println!("{:?} {:?}", (min_x, min_y), (max_x, max_y));

    println!("input: {:?}", sensors.as_slice());
    let part1 = (-5000000..5000000)
        .map(|x| (x, 2000000))
        .map(|pos| {
            sensors
                .iter()
                .map(|s| manhatan_dist(pos, s.sensor) <= s.dist && s.becon != pos)
                .any(|f| f) as usize
        })
        .sum::<usize>();

    println!("PART1: {}", part1);
}
