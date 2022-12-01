fn parse_input(input: &str) -> Vec<u32> {
    let values = {
        let mut values: Vec<_> = input.lines().map(|s| s.parse::<u32>().ok()).collect();

        values.push(None);

        values
    };

    let mut out = vec![];
    let mut current = 0;
    for v in values {
        match v {
            Some(v) => {
                current += v;
            }
            None => {
                out.push(current);
                current = 0;
            }
        }
    }

    out
}

fn solve_part1(input: &str) -> u32 {
    let values = parse_input(input);
    values.iter().max().copied().unwrap()
}

fn solve_part2(input: &str) -> u32 {
    let mut values = parse_input(input);

    values.sort_by_key(|i| std::cmp::Reverse(*i));
    values.iter().take(3).sum()
}

fn main() {
    let input = include_str!("day1_input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
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
        let input = include_str!("day1_input.txt");

        assert_eq!(solve_part1(TEST_INPUT), 24000);
        assert_eq!(solve_part1(input), 67016);

        assert_eq!(solve_part2(TEST_INPUT), 45000);
        assert_eq!(solve_part2(input), 200116);
    }
}
