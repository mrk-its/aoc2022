cd $(dirname $0)
set -e
PROGS="day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12 day13 day14 day17 day18"
# PROGS="day01 day02 day03 day04 day06 day07 day08 day09 day10 day13 day17 day18"
for cmd in "cargo build" "timeout -v 5m cargo run"; do
  for prg in $PROGS; do
    for opts in "--release"; do
        echo $cmd -p $prg $opts
        $cmd -p $prg $opts
    done
  done;
done
