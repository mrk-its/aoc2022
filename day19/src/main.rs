#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

utils::entry!(main);
extern crate alloc;

use alloc::vec::Vec;
use itertools::Itertools;
#[allow(unused_imports)]
use mos_alloc;
use ufmt_stdio::{ufmt::derive::uDebug, *};
use utils::to_str;

#[derive(Default, Clone, uDebug)]
struct Inventory {
    ore_robots: isize,
    clay_robots: isize,
    obsydian_robots: isize,
    geode_robots: isize,
    ore: isize,
    clay: isize,
    obsydian: isize,
    geode: isize,
}
#[derive(Default, Clone, uDebug)]
struct Blueprint {
    id: isize,
    ore_robot_ore: isize,
    clay_robot_ore: isize,
    obsydian_robot_ore: isize,
    obsydian_robot_clay: isize,
    geode_robot_ore: isize,
    geode_robot_obsydian: isize,
    max_ore: isize,
}

const SPACE: &[u8] = b"                              ";

fn turn(blueprint: &Blueprint, inventory: Inventory, t: isize, best_inventory: &mut Inventory) {
    // println!("{} {} {:?}", to_str(&SPACE[0..t as usize]), t, inventory);
    if t == 0 {
        if inventory.geode >= best_inventory.geode {
            // println!("best inventory: {:?}", inventory);
            *best_inventory = inventory;
        }
        return;
    }

    let mut new_inventory = inventory.clone();
    new_inventory.ore += new_inventory.ore_robots;
    new_inventory.clay += new_inventory.clay_robots;
    new_inventory.obsydian += new_inventory.obsydian_robots;
    new_inventory.geode += new_inventory.geode_robots;

    let missing_ore1 = (blueprint.ore_robot_ore - inventory.ore).max(0);
    let missing_ore2 = (blueprint.clay_robot_ore - inventory.ore).max(0);
    let missing_ore3 = (blueprint.obsydian_robot_ore - inventory.ore).max(0);
    let missing_ore4 = (blueprint.geode_robot_ore - inventory.ore).max(0);
    let missing_clay = (blueprint.obsydian_robot_clay - inventory.clay).max(0);
    let missing_obsydian = (blueprint.geode_robot_obsydian - inventory.obsydian).max(0);

    //  check if the current branch could improve on the best solution so far: if you assume it can and will create a geode robot every minute from now on
    // https://www.reddit.com/r/adventofcode/comments/zpy5rm/comment/j0vzi7h/?utm_source=reddit&utm_medium=web2x&context=3
    let may_improve = (inventory.geode + (inventory.geode_robots * (t - 1)) + ((t - 1) * t) / 2)
        > best_inventory.geode;

    let c3 = missing_clay == 0 && missing_ore3 == 0;
    let c4 = missing_obsydian == 0 && missing_ore4 == 0;
    if !(missing_ore1 == 0 && missing_ore2 == 0 && c3 && c4) {
        turn(blueprint, new_inventory.clone(), t - 1, best_inventory);
    }

    if c4 && may_improve {
        let mut inventory = new_inventory.clone();
        inventory.geode += t - 1;
        // inventory.geode_robots += 1;
        inventory.ore -= blueprint.geode_robot_ore;
        inventory.obsydian -= blueprint.geode_robot_obsydian;
        turn(blueprint, inventory, t - 1, best_inventory)
    } else if c3
        && inventory.obsydian + inventory.obsydian_robots * t < t * blueprint.geode_robot_obsydian
        && may_improve
    {
        let mut inventory = new_inventory.clone();
        inventory.obsydian_robots += 1;
        inventory.ore -= blueprint.obsydian_robot_ore;
        inventory.clay -= blueprint.obsydian_robot_clay;
        turn(blueprint, inventory, t - 1, best_inventory);
    } else {
        if missing_ore1 == 0
            && inventory.ore + inventory.ore_robots * t < t * blueprint.max_ore
            && may_improve
        {
            let mut inventory = new_inventory.clone();
            inventory.ore_robots += 1;
            inventory.ore -= blueprint.ore_robot_ore;
            turn(blueprint, inventory, t - 1, best_inventory);
        }
        if missing_ore2 == 0
            && inventory.clay + inventory.clay_robots * t < t * blueprint.obsydian_robot_clay
            && may_improve
        {
            let mut inventory = new_inventory.clone();
            inventory.clay_robots += 1;
            inventory.ore -= blueprint.clay_robot_ore;
            turn(blueprint, inventory, t - 1, best_inventory);
        }
    }
}

fn main() {
    let blueprints = utils::iter_lines!("../../input/day19/input.txt").map(|line| {
        let v = line
            .split(|b| !b.is_ascii_digit())
            .filter(|v| v.len() > 0)
            .map(|v| to_str(v).parse::<isize>().unwrap())
            .collect_tuple::<(_, _, _, _, _, _, _)>()
            .unwrap();
        Blueprint {
            id: v.0,
            ore_robot_ore: v.1,
            clay_robot_ore: v.2,
            obsydian_robot_ore: v.3,
            obsydian_robot_clay: v.4,
            geode_robot_ore: v.5,
            geode_robot_obsydian: v.6,
            max_ore: v.1.max(v.2).max(v.3).max(v.5),
        }
    });
    let mut part1 = 0;
    for blueprint in blueprints.clone() {
        println!("{:?}", blueprint);
        let mut best_inventory = Inventory::default();
        let mut inventory = Inventory::default();
        inventory.ore_robots = 1;

        turn(&blueprint, inventory, 24, &mut best_inventory);
        println!("#{} {:?}", blueprint.id, best_inventory);
        part1 += blueprint.id * best_inventory.geode;
    }
    println!("PART1: {}", part1);

    // let mut part2 = 1;
    // for blueprint in blueprints.clone().take(3) {
    //     let mut best_inventory = Inventory::default();
    //     let mut inventory = Inventory::default();
    //     inventory.ore_robots = 1;
    //     inventory.ore += inventory.ore_robots;
    //     turn(&blueprint, inventory, 31, &mut best_inventory);
    //     println!("#{} {:?}", blueprint.id, best_inventory);
    //     part2 *= best_inventory.geode;
    // }
    // println!("PART2: {}", part2);
    // assert!(part2 == 13475);
}
