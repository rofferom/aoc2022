use std::mem;

const INPUT: &str = include_str!("day11_input.txt");

struct Monkey<'a> {
    inspected_items: usize,

    items: Vec<u64>,

    op_left: &'a str,
    op_operand: &'a str,
    op_right: &'a str,

    test_value: u64,
    test_true_target: usize,
    test_false_target: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|block| {
            let lines: Vec<_> = block.lines().collect();

            // "Starting items: "
            let items: Vec<_> = lines[1][18..]
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect();

            // "Operation: new = "
            let s: Vec<_> = lines[2][19..].split(' ').collect();
            assert_eq!(s.len(), 3);
            let (op_left, op_operand, op_right) = (s[0], s[1], s[2]);

            // "Test: divisible by 23"
            let test_value = lines[3][21..].parse().unwrap();

            // If true/false: throw to monkey 0
            let test_true_target = lines[4][29..].parse().unwrap();
            let test_false_target = lines[5][30..].parse().unwrap();

            Monkey {
                inspected_items: 0,
                items,
                op_left,
                op_operand,
                op_right,
                test_value,
                test_true_target,
                test_false_target,
            }
        })
        .collect()
}

fn solve<C>(mut monkeys: Vec<Monkey>, rounds: usize, round_cb: C) -> usize
where
    C: Fn(u64) -> u64,
{
    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            let monkey = &mut monkeys[idx];

            let item_targets: Vec<_> = mem::take(&mut monkey.items)
                .into_iter()
                .map(|item| {
                    monkey.inspected_items += 1;

                    let get_value = |v: &str| {
                        if v == "old" {
                            item
                        } else {
                            v.parse().unwrap()
                        }
                    };

                    let left = get_value(monkey.op_left);
                    let right = get_value(monkey.op_right);

                    let new_item = match monkey.op_operand {
                        "+" => round_cb(left + right),
                        "*" => round_cb(left * right),
                        _ => panic!(),
                    };

                    let target = if new_item % monkey.test_value == 0 {
                        monkey.test_true_target
                    } else {
                        monkey.test_false_target
                    };

                    (target, new_item)
                })
                .collect();

            for (target, item) in item_targets {
                monkeys[target].items.push(item);
            }
        }
    }

    let mut v: Vec<_> = monkeys
        .into_iter()
        .map(|monkey| monkey.inspected_items)
        .collect();
    v.sort();
    v.into_iter().rev().take(2).product()
}

fn solve_part1(input: &str) -> usize {
    let monkeys = parse(input);
    solve(monkeys, 20, |v| v / 3)
}

fn solve_part2(input: &str) -> usize {
    let monkeys = parse(input);
    let threshold: u64 = monkeys.iter().map(|m| m.test_value).product();
    solve(monkeys, 10000, |v| v % threshold)
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn day11() {
        assert_eq!(solve_part1(TEST_INPUT), 10605);
        assert_eq!(solve_part1(INPUT), 110220);

        assert_eq!(solve_part2(TEST_INPUT), 2713310158);
        assert_eq!(solve_part2(INPUT), 19457438264);
    }
}
