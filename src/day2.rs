use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2, part1)]
fn parse_input_part1(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|range| {
            let mut bounds = range.split('-');
            let start = bounds.next().unwrap().parse::<usize>().unwrap();
            let end = bounds.next().unwrap().parse::<usize>().unwrap();
            (start, end)
        })
        .collect()
}

#[aoc_generator(day2, part2)]
fn parse_input_part2(input: &str) -> Vec<(usize, usize)> {
    parse_input_part1(input)
}

#[aoc(day2, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    let mut invalid_ids: Vec<usize> = Vec::new();
    for (start, end) in input.iter() {
        for id in *start..=*end {
            let id_str = id.to_string();
            // Skip if the length is not divisible by 2.
            if id_str.len() % 2 != 0 {
                continue;
            }
            // Split the ID into two halves. If both halves are the same, it's an invalid ID.
            let mid = id_str.len() / 2;
            let first_half = &id_str[..mid];
            let second_half = &id_str[mid..];
            if first_half == second_half {
                invalid_ids.push(id);
            }
        }
    }
    invalid_ids.iter().sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    let mut invalid_ids: Vec<usize> = Vec::new();
    for (start, end) in input.iter() {
        for id in *start..=*end {
            let id_str = id.to_string();
            let len = id_str.len();
            if len == 1 {
                continue;
            }
            for chunk_size in 1..(len / 2) + 1 {
                if len % chunk_size != 0 {
                    continue;
                }
                let mut is_invalid = true;
                let first_chunk = &id_str[..chunk_size];
                for j in 1..(len / chunk_size) {
                    let start_idx = j * chunk_size;
                    let end_idx = start_idx + chunk_size;
                    if &id_str[start_idx..end_idx] != first_chunk {
                        is_invalid = false;
                        break;
                    }
                }
                if is_invalid {
                    invalid_ids.push(id);
                    break;
                }
            }
        }
    }
    invalid_ids.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
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
        assert_eq!(1227775554, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(4174379265, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
