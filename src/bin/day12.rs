use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("day12_input.txt");

#[derive(Clone)]
struct Node {
    elevation: u32,
    distance: u32,
    edges_idx: Vec<usize>,
}

#[derive(Eq, PartialEq)]
struct NodeRef {
    idx: usize,
    distance: u32,
}

impl Ord for NodeRef {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for NodeRef {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> (Vec<Node>, usize, usize) {
    let mut nodes = vec![];

    let mut start_idx = None;
    let mut end_idx = None;

    let mut rows = 0;

    // Create nodes
    for (y, l) in input.lines().enumerate() {
        rows += 1;

        for (x, c) in l.chars().enumerate() {
            let c = match c {
                'S' => {
                    assert!(start_idx.is_none());
                    start_idx = Some((x, y));

                    'a'
                }
                'E' => {
                    assert!(end_idx.is_none());
                    end_idx = Some((x, y));

                    'z'
                }
                _ => {
                    assert!(c.is_lowercase());
                    c
                }
            };

            nodes.push(Node {
                elevation: c as u32 - 'a' as u32,
                distance: u32::MAX,
                edges_idx: vec![],
            });
        }
    }

    let rows = rows;
    let columns = nodes.len() / rows;

    let get_node_idx = |x: usize, y: usize| y * columns + x;

    // Fill edges
    for i in 0..nodes.len() {
        let node_elevation = nodes[i].elevation;

        let x = (i % columns) as i32;
        let y = (i / columns) as i32;

        let directions = &[
            (x > 0, -1, 0),
            (x + 1 < columns as i32, 1, 0),
            (y > 0, 0, -1),
            (y + 1 < rows as i32, 0, 1),
        ];

        let mut edges: Vec<_> = directions
            .iter()
            .filter_map(|(valid, x_delta, y_delta)| {
                // Filter coordinates outside of the grid and
                // get their final index
                if *valid {
                    Some(get_node_idx((x + x_delta) as usize, (y + y_delta) as usize))
                } else {
                    None
                }
            })
            .filter(|&edge_idx| {
                // Filter the reachable items
                let src = node_elevation;
                let target = nodes[edge_idx].elevation;

                target <= src || src + 1 == target
            })
            .collect();

        nodes[i].edges_idx.append(&mut edges);
    }

    let unpack_idx = |idx: Option<(usize, usize)>| {
        let idx = idx.unwrap();
        get_node_idx(idx.0, idx.1)
    };

    (nodes, unpack_idx(start_idx), unpack_idx(end_idx))
}

fn shortest_path(mut nodes: Vec<Node>, start_idx: usize, end_idx: usize) -> u32 {
    let mut heap: BinaryHeap<_> = BinaryHeap::new();

    nodes[start_idx].distance = 0;

    heap.push(NodeRef {
        idx: start_idx,
        distance: 0,
    });

    while let Some(NodeRef { idx, distance }) = heap.pop() {
        if idx == end_idx {
            break;
        }

        if nodes[idx].distance < distance {
            continue;
        }

        let node_distance = nodes[idx].distance;

        for edge_idx in nodes[idx].edges_idx.clone() {
            let edge = &mut nodes[edge_idx];

            if node_distance + 1 < edge.distance {
                edge.distance = node_distance + 1;

                heap.push(NodeRef {
                    idx: edge_idx,
                    distance: edge.distance,
                });
            }
        }
    }

    nodes[end_idx].distance
}

fn solve_part1(input: &str) -> u32 {
    let (nodes, start_idx, end_idx) = parse_input(input);
    shortest_path(nodes, start_idx, end_idx)
}

fn solve_part2(input: &str) -> u32 {
    let (nodes, _, end_idx) = parse_input(input);

    nodes
        .iter()
        .enumerate()
        .filter_map(
            |(idx, node)| {
                if node.elevation == 0 {
                    Some(idx)
                } else {
                    None
                }
            },
        )
        .map(|start_idx| shortest_path(nodes.clone(), start_idx, end_idx))
        .min()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn day12() {
        assert_eq!(solve_part1(TEST_INPUT), 31);
        assert_eq!(solve_part1(INPUT), 449);

        assert_eq!(solve_part2(TEST_INPUT), 29);
        assert_eq!(solve_part2(INPUT), 443);
    }
}
