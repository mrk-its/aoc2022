#![no_std]
#![feature(start)]

utils::entry!(main);

use ufmt_stdio::*;

const DATA: &[u8] = b"ab";
fn main() {
    if DATA.iter().cycle().skip(2).next().is_some() {
        println!("ok");
    } else {
        println!("err");
    }
}
