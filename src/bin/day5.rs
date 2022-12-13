use std::collections::LinkedList;

const INPUT: &str = include_str!("day5_input.txt");

enum ParseStep {
    Crates,
    Instructions,
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut step = ParseStep::Crates;
    let mut columns: Vec<Vec<char>> = vec![];
    let mut instructions = vec![];

    for l in input.lines() {
        if l.is_empty() {
            step = ParseStep::Instructions;
            continue;
        }

        match step {
            ParseStep::Crates => {
                let blocks_count = l.len() / 4;

                for i in 0..=blocks_count {
                    let c = l.chars().nth(i * 4 + 1).unwrap();
                    if c.is_ascii_digit() {
                        break;
                    }

                    if columns.len() < i + 1 {
                        columns.push(vec![]);
                    }

                    if c != ' ' {
                        columns[i].push(c);
                    }
                }
            }
            ParseStep::Instructions => {
                let s: Vec<_> = l.split(' ').collect();
                let count = s[1].parse().unwrap();
                let from = s[3].parse().unwrap();
                let to = s[5].parse().unwrap();

                instructions.push(Instruction { count, from, to });
            }
        }
    }

    (columns, instructions)
}

fn solve_part1(input: &str) -> String {
    let (columns, instructions) = parse(input);

    let mut columns: Vec<LinkedList<char>> = columns
        .into_iter()
        .map(|c| c.iter().copied().collect())
        .collect();

    for instr in instructions {
        for _ in 0..instr.count {
            let item = columns[instr.from - 1].pop_front().unwrap();
            columns[instr.to - 1].push_front(item);
        }
    }

    columns.into_iter().map(|c| *c.front().unwrap()).collect()
}

fn solve_part2(input: &str) -> String {
    let (mut columns, instructions) = parse(input);

    for c in &mut columns {
        c.reverse();
    }

    for instr in instructions {
        let column = &mut columns[instr.from - 1];
        let mut items = column[column.len() - instr.count..column.len()].to_vec();

        column.resize(column.len() - instr.count, ' ');
        columns[instr.to - 1].append(&mut items);
    }

    columns.into_iter().map(|c| c[c.len() - 1]).collect()
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn day5() {
        assert_eq!(solve_part1(TEST_INPUT), "CMZ");
        assert_eq!(solve_part1(INPUT), "QNNTGTPFN");

        assert_eq!(solve_part2(TEST_INPUT), "MCD");
        assert_eq!(solve_part2(INPUT), "GGNPJBTTR");
    }
}
