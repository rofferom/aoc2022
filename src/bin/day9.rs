use std::cmp::Ordering;
use std::collections::HashSet;

const INPUT: &str = include_str!("day9_input.txt");

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance(&self, o: &Self) -> f64 {
        (((self.x - o.x).pow(2) + (self.y - o.y).pow(2)) as f64).sqrt()
    }
}

fn solve(input: &str, count: usize) -> usize {
    let mut knots = vec![Position::default(); count];
    let mut tail_positions: HashSet<Position> = HashSet::new();

    for l in input.lines() {
        let (dir, count) = l.split_once(' ').unwrap();

        for _ in 0..count.parse::<usize>().unwrap() {
            let head = &mut knots[0];
            match dir {
                "R" => {
                    head.x += 1;
                }
                "L" => {
                    head.x -= 1;
                }
                "U" => {
                    head.y += 1;
                }
                "D" => {
                    head.y -= 1;
                }
                _ => {
                    panic!();
                }
            }

            for idx in 1..knots.len() {
                let prev = knots[idx - 1];
                let cur = &mut knots[idx];

                if cur.distance(&prev) >= 2.0 {
                    cur.x += match prev.x.cmp(&cur.x) {
                        Ordering::Greater => 1,
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                    };

                    cur.y += match prev.y.cmp(&cur.y) {
                        Ordering::Greater => 1,
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                    };
                }
            }

            tail_positions.insert(*knots.last().unwrap());
        }
    }

    tail_positions.len()
}

fn solve_part1(input: &str) -> usize {
    solve(input, 2)
}

fn solve_part2(input: &str) -> usize {
    solve(input, 10)
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn day9() {
        assert_eq!(solve_part1(TEST_INPUT1), 13);
        assert_eq!(solve_part1(INPUT), 6271);

        assert_eq!(solve_part2(TEST_INPUT2), 36);
        assert_eq!(solve_part2(INPUT), 2458);
    }
}
