use std::cmp::Ordering;

const INPUT: &str = include_str!("day13_input.txt");

struct SignalIterator<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> SignalIterator<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, pos: 0 }
    }
}

#[derive(Debug)]
enum Item<'a> {
    List(&'a str),
    Integer(u32),
}

impl<'a> Iterator for SignalIterator<'a> {
    type Item = Item<'a>;

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

                                    return Some(Item::List(&self.s[start..i + 1]));
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

fn cmp_signal(left: &str, right: &str) -> Ordering {
    let mut left_it = SignalIterator::new(left);
    let mut right_it = SignalIterator::new(right);

    loop {
        let ordering = match (left_it.next(), right_it.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,

            (Some(Item::Integer(left)), Some(Item::Integer(right))) => left.cmp(&right),
            (Some(Item::List(left)), Some(Item::List(right))) => {
                cmp_signal(&left[1..left.len() - 1], &right[1..right.len() - 1])
            }
            (Some(Item::List(left)), Some(Item::Integer(right))) => {
                cmp_signal(left, &format!("[{right}]"))
            }
            (Some(Item::Integer(left)), Some(Item::List(right))) => {
                cmp_signal(&format!("[{left}]"), right)
            }
        };

        if ordering != Ordering::Equal {
            return ordering;
        }
    }
}

fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(idx, block)| {
            let (left, right) = block.split_once('\n').unwrap();
            if cmp_signal(left, right) == Ordering::Less {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let dividers = ["[[2]]", "[[6]]"];

    let mut blocks = dividers.to_vec();

    for block in input.split("\n\n") {
        let (left, right) = block.split_once('\n').unwrap();
        blocks.push(left);
        blocks.push(right);
    }

    blocks.sort_by(|left, right| cmp_signal(left, right));

    blocks
        .into_iter()
        .enumerate()
        .filter_map(|(idx, block)| {
            if dividers.contains(&block) {
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
        assert_eq!(cmp_signal("[1,1,3,1,1]", "[1,1,5,1,1]"), Ordering::Less);
        assert_eq!(cmp_signal("[[1],[2,3,4]]", "[[1],4]"), Ordering::Less);
        assert_eq!(cmp_signal("[9]", "[[8,7,6]]"), Ordering::Greater);
        assert_eq!(cmp_signal("[[4,4],4,4]", "[[4,4],4,4,4]"), Ordering::Less);
        assert_eq!(cmp_signal("[7,7,7,7]", "[7,7,7]"), Ordering::Greater);
        assert_eq!(cmp_signal("[]", "[3]"), Ordering::Less);
        assert_eq!(cmp_signal("[[[]]]", "[[]]"), Ordering::Greater);
        assert_eq!(
            cmp_signal("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            Ordering::Greater
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
