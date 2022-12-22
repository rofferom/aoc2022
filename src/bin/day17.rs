use std::{collections::HashSet, hash::Hash};

const INPUT: &str = include_str!("day17_input.txt");

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

const WIDTH: i32 = 7;

fn print_hitmap(hitmap: &HashSet<(i32, i32)>) {
    let max_y = hitmap.iter().map(|(_, y)| y).max().copied().unwrap();

    let max_y = std::cmp::max(max_y, 5);

    for y in (0..=max_y).rev() {
        print!("|");
        for x in 0..7 {
            if hitmap.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }

    println!("\n");
}

fn solve_part1(input: &str) -> i32 {
    let pieces = [
        // ####
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        // .#.
        // ###
        // .#.
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        // ..#
        // ..#
        // ###
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        // #
        // #
        // #
        // #
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        // ##
        // ##
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let directions: Vec<_> = input
        .chars()
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!(),
        })
        .collect();

    let mut hitmap = HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]);

    let mut dir_idx = 0;
    let mut piece_idx = 0;

    for _ in 0..2022 {
        let mut piece = pieces[piece_idx].clone();

        let max_y = hitmap.iter().map(|(_, y)| y).max().unwrap();

        let x_offset = 2;
        let y_offset = max_y + 4;

        for (part_x, part_y) in &mut piece {
            *part_x += x_offset;
            *part_y += y_offset;
        }

        loop {
            // Move Left <-> Right
            let direction = directions[dir_idx];
            dir_idx = (dir_idx + 1) % directions.len();

            let mut move_x = 0;

            match direction {
                Direction::Left => {
                    let mut can_move = true;
                    for (part_x, part_y) in &mut piece {
                        if *part_x - 1 < 0 || hitmap.contains(&(*part_x - 1, *part_y)) {
                            can_move = false;
                            break;
                        }
                    }

                    if can_move {
                        move_x = -1;
                    }
                }
                Direction::Right => {
                    let mut can_move = true;
                    for (part_x, part_y) in &mut piece {
                        if *part_x + 1 == WIDTH || hitmap.contains(&(*part_x + 1, *part_y)) {
                            can_move = false;
                            break;
                        }
                    }

                    if can_move {
                        move_x = 1;
                    }
                }
            }

            if move_x != 0 {
                for (part_x, _) in &mut piece {
                    *part_x += move_x;
                }
            }

            // Fall
            let mut can_fall = true;
            for (part_x, part_y) in &piece {
                if hitmap.contains(&(*part_x, part_y - 1)) {
                    can_fall = false;
                    break;
                }
            }

            if can_fall {
                for (_, part_y) in &mut piece {
                    *part_y -= 1;
                }
            } else {
                for (part_x, part_y) in &piece {
                    hitmap.insert((*part_x, *part_y));
                }

                break;
            }
        }

        //print_hitmap(&hitmap);
        piece_idx = (piece_idx + 1) % pieces.len();
    }

    hitmap.iter().map(|(_, y)| y).max().copied().unwrap()
}

fn solve_part2(input: &str) -> usize {
    0
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17() {
        assert_eq!(solve_part1(TEST_INPUT), 3068);
        assert_eq!(solve_part1(INPUT), 3102);

        //assert_eq!(solve_part2(TEST_INPUT), 56000011);
        //assert_eq!(solve_part2(INPUT), 10884459367718);

        const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    }
}
