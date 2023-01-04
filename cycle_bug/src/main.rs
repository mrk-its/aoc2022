#![no_std]
#![no_main]

#[allow(unused_imports)]
use utils;

#[cfg_attr(not(test), export_name = "main")]
#[cfg_attr(test, allow(dead_code))]
fn main() {}

#[mos_test::tests]
mod tests {
    #[test]
    pub fn cycle_bug() {
        const DATA: &[u8] = b"ab";

        assert_eq!(DATA.iter().cycle().skip(2).next(), Some(&b'a'));
    }
}
