use std::cmp;
use std::collections::HashSet;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("day15_input.txt");

struct Position {
    x: i32,
    y: i32,
}

struct Pair {
    sensor: Position,
    beacon: Position,
    distance: i32,
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(": ").unwrap();

            let parse = |l: &str| {
                let (x, y) = l.split_once(", y=").unwrap();

                Position {
                    x: x.parse::<i32>().unwrap(),
                    y: y.parse::<i32>().unwrap(),
                }
            };

            let sensor = parse(&left[12..]);
            let beacon = parse(&right[23..]);
            let distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

            Pair {
                sensor,
                beacon,
                distance,
            }
        })
        .collect()
}

fn get_ranges_for_line(pairs: &Vec<Pair>, target_y: i32) -> Vec<RangeInclusive<i32>> {
    let mut ranges = HashSet::new();

    // Build the ranges of the requested line
    for pair in pairs {
        if pair.sensor.y == target_y {
            ranges.insert(pair.sensor.x..=pair.sensor.x);
        }

        if pair.beacon.y == target_y {
            ranges.insert(pair.beacon.x..=pair.beacon.x);
        }

        // Filter sensors that doesn't reach the target line
        let min_y = pair.sensor.y - pair.distance;
        let max_y = pair.sensor.y + pair.distance;
        if !(min_y..=max_y).contains(&target_y) {
            continue;
        }

        // Fill the line
        let width = pair.distance - (target_y - pair.sensor.y).abs();

        ranges.insert(pair.sensor.x - width..=pair.sensor.x + width);
    }

    let mut ranges: Vec<_> = ranges.into_iter().collect();

    // Merge the ranges to contigous ones
    loop {
        let initial_ranges_count = ranges.len();
        let mut unmerged_ranges = ranges;
        ranges = vec![];

        while !unmerged_ranges.is_empty() {
            let mut front = unmerged_ranges.remove(0);

            let mut to_remove = vec![];

            for (idx, range) in unmerged_ranges.iter().enumerate() {
                if front.end() < range.start() || range.end() < front.start() {
                    continue;
                }

                if range.start() < front.start() && front.end() >= range.end() {
                    front = *range.start()..=*front.end();
                    to_remove.push(idx);
                } else if front.start() <= range.start() && range.end() > front.end() {
                    front = *front.start()..=*range.end();
                    to_remove.push(idx);
                } else if range.start() <= front.start() && front.end() <= range.end() {
                    front = range.clone();
                    to_remove.push(idx);
                } else if front.start() <= range.start() && range.end() <= front.end() {
                    to_remove.push(idx);
                } else {
                    panic!();
                }
            }

            ranges.push(front);

            for idx in to_remove.iter().rev() {
                unmerged_ranges.remove(*idx);
            }
        }

        // No merge detected
        if ranges.len() == initial_ranges_count {
            break;
        }
    }

    ranges
}

fn get_segment_items_sum(pairs: &Vec<Pair>, max_x: i32, target_y: i32) -> i32 {
    let ranges = get_ranges_for_line(pairs, target_y);

    ranges
        .into_iter()
        .filter_map(|r| {
            if *r.end() < 0 || *r.start() > max_x {
                None
            } else {
                let start = cmp::max(0, *r.start()) + 1;
                let end = cmp::min(max_x, *r.end()) + 1;

                Some(start..=end)
            }
        })
        .map(|r| ((*r.end() * (*r.end() + 1)) / 2) - ((*r.start() * (*r.start() - 1)) / 2))
        .sum()
}

fn solve_part1(input: &str, target_y: i32) -> i32 {
    let pairs = parse_input(input);
    let ranges = get_ranges_for_line(&pairs, target_y);

    ranges.iter().map(|r| r.end() - r.start()).sum()
}

fn solve_part2(input: &str, dim: i32) -> usize {
    let pairs = parse_input(input);

    let line_sum = (dim + 1) * (dim + 2) / 2;

    for y in 0..=dim {
        let sum = get_segment_items_sum(&pairs, dim, y);
        if sum != line_sum {
            let x = line_sum - sum - 1;
            return x as usize * 4000000 + y as usize;
        }
    }

    panic!()
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT, 2000000));
    println!("Part 2: {}", solve_part2(INPUT, 4000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15() {
        assert_eq!(solve_part1(TEST_INPUT, 11), 26);
        assert_eq!(solve_part1(INPUT, 2000000), 5142231);

        assert_eq!(solve_part2(TEST_INPUT, 20), 56000011);
        assert_eq!(solve_part2(INPUT, 4000000), 10884459367718);

        const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    }
}
