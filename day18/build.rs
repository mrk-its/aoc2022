use std::{env, fs, path::Path};

fn main() {
    let mut parsed = include_str!("../input/day18/input.txt")
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|v| v.parse::<i8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    parsed.sort();
    let out = parsed
        .iter()
        .map(|v| {
            format!(
                "({})",
                v.iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(","),
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let contents = format!("pub const POINTS: &[(i8, i8, i8)] = &[{}];", out);
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("input.rs");

    fs::write(&dest_path, contents).unwrap();
}
