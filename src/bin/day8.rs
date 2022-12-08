use std::collections::HashSet;

const INPUT: &str = include_str!("day8_input.txt");

fn parse(input: &str) -> (Vec<Vec<i32>>, usize, usize) {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    (grid, rows, cols)
}

fn iter_cols<R>(grid: &[Vec<i32>], row: usize, range: R) -> Vec<(usize, usize)>
where
    R: IntoIterator<Item = usize>,
{
    let mut height = -1;
    let mut visible = vec![];

    for col in range {
        if grid[row][col] > height {
            height = grid[row][col];
            visible.push((row, col));
        }
    }

    visible
}

fn iter_rows<R>(grid: &[Vec<i32>], col: usize, range: R) -> Vec<(usize, usize)>
where
    R: IntoIterator<Item = usize>,
{
    let mut height = -1;
    let mut visible = vec![];

    for row in range {
        if grid[row][col] > height {
            height = grid[row][col];
            visible.push((row, col));
        }
    }

    visible
}

fn solve_part1(input: &str) -> usize {
    let (grid, rows, cols) = parse(input);
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for row in 0..rows {
        visible.extend(iter_cols(&grid, row, 0..cols).into_iter());
        visible.extend(iter_cols(&grid, row, (0..cols).rev()).into_iter());
    }

    for col in 0..cols {
        visible.extend(iter_rows(&grid, col, 0..rows).into_iter());
        visible.extend(iter_rows(&grid, col, (0..rows).rev()).into_iter());
    }

    visible.len()
}

fn solve_part2(input: &str) -> u32 {
    let (grid, rows, cols) = parse(input);

    let mut max_score = 0;

    for start_y in 1..rows - 1 {
        for start_x in 1..cols - 1 {
            let start_height = grid[start_y][start_x];

            let score = [(0, -1), (-1, 0), (0, 1), (1, 0)]
                .into_iter()
                .map(|(inc_x, inc_y)| {
                    let mut count = 0;
                    let (mut x, mut y) = (start_x as i32 + inc_x, start_y as i32 + inc_y);

                    loop {
                        if !(0..cols as i32).contains(&x) || !(0..rows as i32).contains(&y) {
                            break;
                        }

                        count += 1;

                        if grid[y as usize][x as usize] < start_height {
                            x += inc_x;
                            y += inc_y;
                        } else {
                            break;
                        }
                    }

                    count
                })
                .product();

            max_score = std::cmp::max(max_score, score);
        }
    }

    max_score
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

        assert_eq!(solve_part1(TEST_INPUT), 21);
        assert_eq!(solve_part1(INPUT), 1662);

        assert_eq!(solve_part2(TEST_INPUT), 8);
        assert_eq!(solve_part2(INPUT), 537600);
    }
}
