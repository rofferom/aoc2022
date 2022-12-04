use std::str::FromStr;

const INPUT: &str = include_str!("day4_input.txt");

#[derive(Clone, Copy, Debug)]
struct Range {
    begin: u32,
    end: u32,
}

impl Range {
    fn inside(&self, n: u32) -> bool {
        self.begin <= n && n <= self.end
    }

    fn contains(&self, r: &Range) -> bool {
        self.begin <= r.begin && r.end <= self.end
    }

    fn overlaps(&self, r: &Range) -> bool {
        self.contains(r)
            || r.contains(self)
            || (self.inside(r.begin) && !self.inside(r.end))
            || (!self.inside(r.begin) && self.inside(r.end))
    }
}

impl std::str::FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l: Vec<_> = s.split('-').map(|i| u32::from_str(i).unwrap()).collect();
        assert_eq!(l.len(), 2);

        Ok(Range {
            begin: l[0],
            end: l[1],
        })
    }
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|l| {
            let s: Vec<_> = l.split(',').map(|i| Range::from_str(i).unwrap()).collect();
            assert_eq!(s.len(), 2);
            (s[0], s[1])
        })
        .collect()
}

fn find_assignments<F>(input: &str, f: F) -> u32
where
    F: Fn(Range, Range) -> bool,
{
    parse_input(input)
        .into_iter()
        .fold(0, |acc, (first, second)| acc + u32::from(f(first, second)))
}

fn solve_part1(input: &str) -> u32 {
    find_assignments(input, |first, second| {
        first.contains(&second) || second.contains(&first)
    })
}

fn solve_part2(input: &str) -> u32 {
    find_assignments(input, |first, second| first.overlaps(&second))
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
    fn day1() {
        assert_eq!(solve_part1(TEST_INPUT), 2);
        assert_eq!(solve_part1(INPUT), 524);

        assert_eq!(solve_part2(TEST_INPUT), 4);
        assert_eq!(solve_part2(INPUT), 798);
    }
}