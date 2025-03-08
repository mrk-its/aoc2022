#![no_std]
#![no_main]
use ufmt_stdio::*;
use utils;

// #[cfg_attr(test, export_name = "main_main")]
#[cfg_attr(not(test), export_name = "main")]
#[cfg_attr(test, allow(dead_code))]
fn main() -> isize {
    type Int = i32;
    let mut input = utils::iter_lines!("../../input/day01/input.txt");
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
    assert_eq!(part1, 69626);
    ufmt_stdio::println!("PART1: {}", part1);

    let part2 = maxes.iter().sum::<Int>();
    assert_eq!(part2, 206780);
    ufmt_stdio::println!("PART2: {}", part2);
    0
}

pub fn assert<E: uDisplay>(expr: bool, v: E) -> Result<(), E> {
    expr.then_some(()).ok_or(v)
}
#[allow(unused_imports)]
#[mos_test::tests]
mod tests {
    use super::*;
    use ufmt_stdio::*;

    #[test]
    fn test() -> Result<&'static str, &'static str> {
        #[cfg(feature = "no_std")]
        println!("no std!");

        assert(1 == 2, "foo")?;
        Ok("ok")
    }
    #[test]
    fn test2() {
        println!("test2");
        assert_eq!(2, 2, "foo");
    }
}
