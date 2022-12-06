use std::collections::HashSet;

const INPUT: &str = include_str!("day3_input.txt");

fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let first: HashSet<_> = l[0..l.len() / 2].chars().collect();
            let second: HashSet<_> = l[l.len() / 2..].chars().collect();

            let intersection: Vec<_> = first.intersection(&second).copied().collect();
            assert_eq!(intersection.len(), 1);

            get_priority(intersection[0])
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;

    for i in 0..lines.len() / 3 {
        let elves: Vec<HashSet<_>> = lines[i * 3..(i + 1) * 3]
            .iter()
            .map(|&elve| elve.chars().collect())
            .collect();
        assert_eq!(elves.len(), 3);

        let intersection: HashSet<_> = elves[0].intersection(&elves[1]).copied().collect();
        let intersection: Vec<_> = intersection.intersection(&elves[2]).copied().collect();
        assert_eq!(intersection.len(), 1);

        sum += get_priority(intersection[0]);
    }

    sum
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn day3() {
        assert_eq!(solve_part1(TEST_INPUT), 157);
        assert_eq!(solve_part1(INPUT), 7793);

        assert_eq!(solve_part2(TEST_INPUT), 70);
        assert_eq!(solve_part2(INPUT), 2499);
    }
}
