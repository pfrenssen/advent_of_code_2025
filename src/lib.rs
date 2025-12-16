use aoc_runner_derive::aoc_lib;

mod day1;
mod day2;

aoc_lib! { year = 2025 }

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}
