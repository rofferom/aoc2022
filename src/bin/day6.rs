use std::collections::HashSet;

const INPUT: &str = include_str!("day6_input.txt");

fn detect_sequence_start(input: &str, len: usize) -> u32 {
    for i in 0..input.len() - len - 1 {
        let s: HashSet<_> = input[i..i + len].chars().collect();
        if s.len() == len {
            return (i + len) as u32;
        }
    }

    panic!();
}

fn solve_part1(input: &str) -> u32 {
    detect_sequence_start(input, 4)
}

fn solve_part2(input: &str) -> u32 {
    detect_sequence_start(input, 14)
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        assert_eq!(solve_part1(INPUT), 1766);

        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
        assert_eq!(solve_part2(INPUT), 2383);
    }
}
