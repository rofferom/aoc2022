use std::cmp::Ordering;

const INPUT: &str = include_str!("day13_input.txt");

struct SignalIterator {
    s: String,
    pos: usize,
}

impl SignalIterator {
    fn new(s: &str) -> Self {
        Self {
            s: s.into(),
            pos: 0,
        }
    }
}

#[derive(Debug)]
enum Item {
    List(String),
    Integer(u32),
}

impl Iterator for SignalIterator {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.s.len() {
            return None;
        }

        loop {
            match self.s.chars().nth(self.pos).unwrap() {
                '[' => {
                    let mut c = 1;

                    for i in self.pos + 1..self.s.len() {
                        match self.s.chars().nth(i).unwrap() {
                            '[' => {
                                c += 1;
                            }
                            ']' => {
                                c -= 1;

                                if c == 0 {
                                    let start = self.pos;
                                    self.pos += i - self.pos + 2;

                                    return Some(Item::List(self.s[start..i + 1].to_string()));
                                }
                            }
                            _ => {}
                        }
                    }
                }
                '0'..='9' => {
                    let value = match self.s[self.pos..].find(',') {
                        Some(end) => {
                            let value: u32 = self.s[self.pos..self.pos + end].parse().unwrap();
                            self.pos += end + 1;

                            value
                        }
                        None => {
                            let value: u32 = self.s[self.pos..].parse().unwrap();
                            self.pos = self.s.len();

                            value
                        }
                    };

                    return Some(Item::Integer(value));
                }
                _ => {}
            }
        }
    }
}

fn is_order_valid(left: &str, right: &str) -> Option<bool> {
    let mut left_it = SignalIterator::new(left);
    let mut right_it = SignalIterator::new(right);

    loop {
        match (left_it.next(), right_it.next()) {
            (Some(Item::Integer(left)), Some(Item::Integer(right))) => match left.cmp(&right) {
                Ordering::Less => return Some(true),
                Ordering::Greater => return Some(false),
                Ordering::Equal => {}
            },
            (Some(Item::List(left)), Some(Item::List(right))) => {
                let valid = is_order_valid(&left[1..left.len() - 1], &right[1..right.len() - 1]);
                if valid.is_some() {
                    return valid;
                }
            }
            (Some(Item::List(left)), Some(Item::Integer(right))) => {
                let valid = is_order_valid(&left, &format!("[{right}]"));
                if valid.is_some() {
                    return valid;
                }
            }
            (Some(Item::Integer(left)), Some(Item::List(right))) => {
                let valid = is_order_valid(&format!("[{left}]"), &right);
                if valid.is_some() {
                    return valid;
                }
            }
            (None, Some(_)) => {
                return Some(true);
            }
            (Some(_), None) => {
                return Some(false);
            }
            (None, None) => {
                return None;
            }
        }
    }
}

fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(idx, block)| {
            let (left, right) = block.split_once('\n').unwrap();
            if let Some(true) = is_order_valid(left, right) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let dividers = ["[[2]]", "[[6]]"];

    let mut blocks: Vec<String> = dividers.iter().map(|s| s.to_string()).collect();

    for block in input.split("\n\n") {
        let (left, right) = block.split_once('\n').unwrap();
        blocks.push(left.into());
        blocks.push(right.into());
    }

    blocks.sort_by(|a, b| {
        if is_order_valid(a, b).unwrap() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    blocks
        .into_iter()
        .enumerate()
        .filter_map(|(idx, block)| {
            if dividers.contains(&block.as_str()) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .product()
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13() {
        assert!(is_order_valid("[1,1,3,1,1]", "[1,1,5,1,1]").unwrap());
        assert!(is_order_valid("[[1],[2,3,4]]", "[[1],4]").unwrap());
        assert!(!is_order_valid("[9]", "[[8,7,6]]").unwrap());
        assert!(is_order_valid("[[4,4],4,4]", "[[4,4],4,4,4]").unwrap());
        assert!(!is_order_valid("[7,7,7,7]", "[7,7,7]").unwrap());
        assert!(is_order_valid("[]", "[3]").unwrap());
        assert!(!is_order_valid("[[[]]]", "[[]]").unwrap());
        assert!(
            !is_order_valid("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap()
        );

        assert_eq!(solve_part1(TEST_INPUT), 13);
        assert_eq!(solve_part1(INPUT), 6101);

        assert_eq!(solve_part2(TEST_INPUT), 140);
        assert_eq!(solve_part2(INPUT), 21909);

        const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    }
}