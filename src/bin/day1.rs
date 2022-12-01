const INPUT: &str = include_str!("day1_input.txt");

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|block| block.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect()
}

fn solve_part1(input: &str) -> u32 {
    let values = parse_input(input);
    values.into_iter().max().unwrap()
}

fn solve_part2(input: &str) -> u32 {
    let mut values = parse_input(input);

    values.sort_by_key(|i| std::cmp::Reverse(*i));
    values.into_iter().take(3).sum()
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn day1() {
        assert_eq!(solve_part1(TEST_INPUT), 24000);
        assert_eq!(solve_part1(INPUT), 67016);

        assert_eq!(solve_part2(TEST_INPUT), 45000);
        assert_eq!(solve_part2(INPUT), 200116);
    }
}
