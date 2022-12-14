use std::{env, fs, path::Path};

fn main() {
    let parsed = include_str!("../input/day14/input.txt")
        .split("\n")
        .map(|line| {
            format!(
                "&[{}]",
                line.split(" -> ")
                    .map(|s| format!("({})", s))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let contents = format!("pub const POINTS: &[&[(i16, i16)]] = &[{}];", parsed);
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("input.rs");

    fs::write(&dest_path, contents).unwrap();
}
