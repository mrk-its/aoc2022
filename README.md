# Rust solutions of Advent of Code 2022, targeting MOS-6502

# Build instructions
Repository [doesn't contain input files](https://www.reddit.com/r/adventofcode/comments/zh2hk0/2022friendly_reminder_dont_commit_your_input/), input data is read from `input/day??/input.txt` locations.

The recommended way of building for 8-bit targets is opening this project in VSCode [devcontainer](https://code.visualstudio.com/docs/devcontainers/containers) (required configuration is included). To run day01 on llvm-mos 6502 simulator, execute following in VSCode terminal (inside of dev container):
```
cargo run -p day01 --release
```
To build executable for atari800 do:
```
cargo build -p day01 --target mos-atari8-none --release
```
Atari executable file will land is `target/mos-atari8-none/release/day01`

Project contains Makefile which may be used to build all solutions for Atari800 with single `make` command (all input files are required in `input/day??` directories). For each day atari `.xex` and `.atr` files are created in `target/mos-atari8-none/release/` directory.

Native executable may be built without devcontainer, but nightly Rust is required:
```
cargo +nightly run -p day01 --target x86_64-unknown-linux-gnu
```
