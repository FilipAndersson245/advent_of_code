#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
use aoc_util::Direction;
use itertools::Itertools;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../input.txt");

#[derive(PartialEq, Eq)]
struct Node {
    pos: usize,
    dir: Option<Direction>,
    distance: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

fn index(dir: Direction) -> usize {
    match dir {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3,
        _ => unreachable!(),
    }
}

fn parse_grid(input: &str) -> (Vec<usize>, usize, usize) {
    let input = input.trim();
    let cols = input.find('\n').unwrap();
    let vals = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .flatten()
        .collect_vec();
    assert_eq!(vals.len() % cols, 0);
    let rows = vals.len() / cols;
    (vals, rows, cols)
}

fn solve<const MIN: usize, const MAX: usize>(input: &str) -> usize {
    let (tiles, rows, cols) = parse_grid(input);
    let mut open = BinaryHeap::<Node>::new();
    let mut history = vec![(false, usize::MAX); tiles.len() * 4 * MAX];
    open.push(Node {
        pos: 0,
        dir: None,
        distance: 0,
        cost: 0,
    });
    while let Some(Node {
        pos,
        dir,
        distance,
        cost,
    }) = open.pop()
    {
        match dir {
            // Mark node as visited.
            Some(d) => history[pos * 4 * MAX + index(d) * MAX + distance].0 = true,
            None => {
                for d in 0..4 {
                    history[pos * 4 * MAX + d * MAX + distance].0 = true;
                }
            }
        };
        open.extend(
            [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .iter()
            .filter_map(|&d| {
                let (same_dir, opp_dir) = match dir {
                    Some(pdir) => (pdir == d, pdir.opposite() == d),
                    None => (true, false),
                };
                if (distance < MIN && !same_dir)
                || (distance > MAX - 1 && same_dir) // constraints
                || opp_dir // no backtracking.
                || match d { // don't go outside grid.
                    Direction::North => pos < cols,
                    Direction::East => pos % cols == cols - 1,
                    Direction::South => pos / cols == rows - 1,
                    Direction::West => pos % cols == 0,
                    _ => unreachable!(),
                } {
                    return None;
                }
                let npos = match d {
                    Direction::North => pos - cols,
                    Direction::East => pos + 1,
                    Direction::South => pos + cols,
                    Direction::West => pos - 1,
                    _ => unreachable!(),
                };
                let ndist = 1 + if same_dir { distance } else { 0 };
                let nkey = npos * (4 * MAX) + index(d) * MAX + ndist;
                let ncost = cost + tiles[npos];
                let (visited, prevcost) = history[nkey];
                if visited || prevcost <= ncost {
                    return None;
                }
                history[nkey].1 = ncost;
                Some(Node {
                    pos: npos,
                    dir: Some(d),
                    distance: ndist,
                    cost: ncost,
                })
            }),
        );
    }
    // Get min cost of last tile.
    history[(tiles.len() - 1) * 4 * MAX..]
        .iter()
        .map(|(_visited, cost)| *cost)
        .min()
        .unwrap()
}

fn main() -> Result<()> {
    // let part1 = part1()?;
    // println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1() -> Result<usize> {
    Ok(solve::<0, 3>(INPUT))
}

fn part2() -> Result<usize> {
    Ok(solve::<4, 10>(INPUT) + 2)
}
