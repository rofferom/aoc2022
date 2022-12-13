const INPUT: &str = include_str!("day10_input.txt");

fn gen_cycles(input: &str) -> Vec<i32> {
    let mut cycles_history = vec![];
    let mut x = 1;

    for l in input.lines() {
        let s: Vec<_> = l.split(' ').collect();

        let opcode = s[0];
        let value = if s.len() == 2 {
            Some(s[1].parse::<i32>().unwrap())
        } else {
            None
        };

        let mut values = match opcode {
            "noop" => {
                vec![x]
            }
            "addx" => {
                let values = vec![x, x];

                x += value.unwrap();

                values
            }
            _ => {
                panic!();
            }
        };

        cycles_history.append(&mut values);
    }

    cycles_history
}

fn solve_part1(input: &str) -> i32 {
    const INTERESTING_CYCLES: &[i32] = &[20, 60, 100, 140, 180, 220];

    gen_cycles(input)
        .into_iter()
        .enumerate()
        .fold(0, |acc, (cycle, value)| {
            let cycle = cycle as i32 + 1;

            if INTERESTING_CYCLES.contains(&cycle) {
                acc + cycle * value
            } else {
                acc
            }
        })
}

fn solve_part2(input: &str) -> String {
    const LINE_LEN: usize = 40;
    let cycles_history = gen_cycles(input);

    let mut out = String::new();

    for (position, &x) in cycles_history.iter().enumerate() {
        if (x - 1..=x + 1).contains(&((position % LINE_LEN) as i32)) {
            out += "#";
        } else {
            out += ".";
        }

        if position != cycles_history.len() - 1 && (position + 1) % LINE_LEN == 0 {
            out += "\n";
        }
    }

    out
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2:\n{}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10() {
        assert_eq!(solve_part1(TEST_INPUT), 13140);
        assert_eq!(solve_part1(INPUT), 14560);

        let part2_test = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        let part2 = "####.#..#.###..#..#.####.###..#..#.####.
#....#.#..#..#.#..#.#....#..#.#..#....#.
###..##...#..#.####.###..#..#.#..#...#..
#....#.#..###..#..#.#....###..#..#..#...
#....#.#..#.#..#..#.#....#....#..#.#....
####.#..#.#..#.#..#.####.#.....##..####.";

        assert_eq!(solve_part2(TEST_INPUT), part2_test);
        assert_eq!(solve_part2(INPUT), part2);
    }

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
