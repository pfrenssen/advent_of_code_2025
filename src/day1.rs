use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1, part1)]
fn parse_input_part1(input: &str) -> Vec<Option<Rotation>> {
    input.lines().map(Rotation::from_str).collect()
}

#[aoc_generator(day1, part2)]
fn parse_input_part2(input: &str) -> Vec<Option<Rotation>> {
    parse_input_part1(input)
}

#[aoc(day1, part1)]
fn part1(input: &[Option<Rotation>]) -> usize {
    let mut dial = 50;
    let mut zeroes = 0;

    for rotation in input.iter().flatten() {
        match rotation.direction {
            Direction::Left => {
                dial = (dial + 100 - (rotation.value as usize % 100)) % 100;
            }
            Direction::Right => {
                dial = (dial + (rotation.value as usize % 100)) % 100;
            }
        }
        if dial == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

#[aoc(day1, part2)]
fn part2(input: &[Option<Rotation>]) -> usize {
    let mut dial = 50;
    let mut zeroes = 0;

    for rotation in input.iter().flatten() {
        // Every full rotation passes zero.
        let full_rotations = rotation.value as usize / 100;
        let remaining_steps = rotation.value as usize % 100;
        match rotation.direction {
            Direction::Left => {
                // If we turn left more than the current dial position, we pass zero.
                if dial != 0 && remaining_steps > dial {
                    zeroes += 1;
                }
                dial = (dial + 100 - remaining_steps) % 100;
            }
            Direction::Right => {
                // If we turn right enough to go past 100, we pass zero once more.
                if dial + remaining_steps > 100 {
                    zeroes += 1;
                }
                dial = (dial + remaining_steps) % 100;
            }
        }
        if dial == 0 {
            if remaining_steps == 0 {
                // Edge case: if we start and end at zero we did full rotations only. Do not double
                // count.
            } else {
                zeroes += 1;
            }
        }
        zeroes += full_rotations;
    }

    zeroes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rotation {
    direction: Direction,
    value: isize,
}

impl Rotation {
    fn from_str(s: &str) -> Option<Rotation> {
        let (dir_str, value_str) = s.split_at(1);
        let direction = Direction::from_str(dir_str)?;
        let value = value_str.parse().ok()?;
        Some(Rotation { direction, value })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Option<Direction> {
        match s {
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    #[rustfmt::skip]
    fn test_parse_input_part1() {
        let expected = vec![
            Some(Rotation { direction: Direction::Left, value: 68 }),
            Some(Rotation { direction: Direction::Left, value: 30 }),
            Some(Rotation { direction: Direction::Right, value: 48 }),
            Some(Rotation { direction: Direction::Left, value: 5 }),
            Some(Rotation { direction: Direction::Right, value: 60 }),
            Some(Rotation { direction: Direction::Left, value: 55 }),
            Some(Rotation { direction: Direction::Left, value: 1 }),
            Some(Rotation { direction: Direction::Left, value: 99 }),
            Some(Rotation { direction: Direction::Right, value: 14 }),
            Some(Rotation { direction: Direction::Left, value: 82 }),
        ];

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));
    }

    #[test]
    fn test_parse_input_part2() {
        test_parse_input_part1();
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(3, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(6, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
