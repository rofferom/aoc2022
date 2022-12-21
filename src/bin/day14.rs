use std::cmp;
use std::collections::HashMap;

const INPUT: &str = include_str!("day14_input.txt");

#[derive(Clone, Copy, Debug, PartialEq)]
enum Type {
    Rock,
    Sand,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

fn parse_input(input: &str) -> (HashMap<Position, Type>, u32) {
    let slices: Vec<Vec<Position>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    Position {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect();

    let max_y = slices
        .iter()
        .flat_map(|s| s.iter().map(|p| p.y))
        .max()
        .unwrap();

    let mut grid = HashMap::new();

    for slice in slices {
        for i in 0..slice.len() - 1 {
            let from = &slice[i];
            let to = &slice[i + 1];

            if from.x == to.x {
                // Vertical line
                let start = cmp::min(from.y, to.y);
                let end = cmp::max(from.y, to.y);

                for y in start..=end {
                    grid.insert(Position { x: from.x, y }, Type::Rock);
                }
            } else {
                // Horizontal line
                let start = cmp::min(from.x, to.x);
                let end = cmp::max(from.x, to.x);

                for x in start..=end {
                    grid.insert(Position { x, y: from.y }, Type::Rock);
                }
            }
        }
    }

    (grid, max_y)
}

fn solve_part1(input: &str) -> usize {
    let (mut grid, max_y) = parse_input(input);

    'outer: loop {
        let (mut x, mut y) = (500, 0);

        loop {
            if grid.get(&Position { x, y: y + 1 }).is_some() {
                if grid.get(&Position { x: x - 1, y: y + 1 }).is_none() {
                    x -= 1;
                } else if grid.get(&Position { x: x + 1, y: y + 1 }).is_none() {
                    x += 1;
                } else {
                    grid.insert(Position { x, y }, Type::Sand);
                    break;
                }
            }

            y += 1;
            if y > max_y {
                break 'outer;
            }
        }
    }

    grid.into_iter().filter(|(_, v)| *v == Type::Sand).count()
}

fn solve_part2(input: &str) -> usize {
    let (mut grid, max_y) = parse_input(input);
    let max_y = max_y + 2;

    let get_grid_entry = |grid: &mut HashMap<Position, Type>, x, y| -> Option<Type> {
        if y < max_y {
            grid.get(&Position { x, y }).copied()
        } else {
            Some(Type::Rock)
        }
    };

    let source = Position { x: 500, y: 0 };
    while grid.get(&source).is_none() {
        let mut pos = source.clone();

        loop {
            if get_grid_entry(&mut grid, pos.x, pos.y + 1).is_some() {
                if get_grid_entry(&mut grid, pos.x - 1, pos.y + 1).is_none() {
                    pos.x -= 1;
                } else if get_grid_entry(&mut grid, pos.x + 1, pos.y + 1).is_none() {
                    pos.x += 1;
                } else {
                    grid.insert(pos, Type::Sand);
                    break;
                }
            }

            pos.y += 1;
        }
    }

    grid.into_iter().filter(|(_, v)| *v == Type::Sand).count()
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14() {
        assert_eq!(solve_part1(TEST_INPUT), 24);
        assert_eq!(solve_part1(INPUT), 674);

        assert_eq!(solve_part2(TEST_INPUT), 93);
        assert_eq!(solve_part2(INPUT), 24958);

        const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    }
}
