#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
use aoc_util::Direction;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

pub fn part1() -> Result<usize> {
    let res = solve(INPUT.lines().map(|line| {
        let mut chars = line.split_ascii_whitespace();

        let direction = match chars.next().unwrap() {
            "U" => Direction::North,
            "D" => Direction::South,
            "L" => Direction::West,
            "R" => Direction::East,
            _ => unimplemented!(),
        };
        let length: usize = chars.next().unwrap().parse().unwrap();
        (length, direction)
    }));

    Ok(res)
}

pub fn part2() -> Result<usize> {
    let res = solve(INPUT.lines().map(|line| {
        let (_, hex) = line.split_once('#').unwrap();
        let mut chars = hex.chars();

        let mut length: usize = 0;
        for _ in 0..5 {
            length = length * 16 + chars.next().unwrap().to_digit(16).unwrap() as usize;
        }

        let direction = match chars.next().unwrap() {
            '3' => Direction::North,
            '1' => Direction::South,
            '2' => Direction::West,
            '0' => Direction::East,
            _ => panic!("Oh no"),
        };

        (length, direction)
    }));

    Ok(res)
}

fn solve<T>(iter: T) -> usize
where
    T: Iterator<Item = (usize, Direction)>,
{
    let mut current: (isize, isize) = (0, 0);
    let mut prev: (isize, isize) = (0, 0);

    let mut count: usize = 0;
    let mut s: isize = 0;

    for (length, direction) in iter {
        match direction {
            Direction::North => current.0 -= length as isize,
            Direction::South => current.0 += length as isize,
            Direction::West => current.1 -= length as isize,
            Direction::East => current.1 += length as isize,
            _ => unreachable!(),
        }

        s += current.0 * prev.1 - current.1 * prev.0;
        count += length;
        prev = current;
    }

    s.abs() as usize / 2 + count / 2 + 1
}
