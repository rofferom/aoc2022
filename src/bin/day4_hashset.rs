use std::collections::HashSet;

const INPUT: &str = include_str!("day4_input.txt");

fn find_assignments<F>(input: &str, f: F) -> u32
where
    F: Fn(HashSet<u32>, HashSet<u32>) -> bool,
{
    input
        .lines()
        .map(|l| {
            let split = |s: &str| -> HashSet<u32> {
                let (begin, end) = s.split_once('-').unwrap();
                (begin.parse().unwrap()..=end.parse().unwrap())
                    .into_iter()
                    .collect()
            };

            let (first, second) = l.split_once(',').unwrap();
            (split(first), split(second))
        })
        .fold(0, |acc, (first, second)| acc + u32::from(f(first, second)))
}

fn solve_part1(input: &str) -> u32 {
    find_assignments(input, |first, second| {
        first.is_subset(&second) || second.is_subset(&first)
    })
}

fn solve_part2(input: &str) -> u32 {
    find_assignments(input, |first, second| !first.is_disjoint(&second))
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn day4_bis() {
        assert_eq!(solve_part1(TEST_INPUT), 2);
        assert_eq!(solve_part1(INPUT), 524);

        assert_eq!(solve_part2(TEST_INPUT), 4);
        assert_eq!(solve_part2(INPUT), 798);
    }
}
