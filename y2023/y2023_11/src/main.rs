#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
use aoc_util::Point;
use indexmap::IndexSet;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1() -> Result<usize> {
    let sum_res = solve(2);
    Ok(sum_res as usize)
}

fn part2() -> Result<usize> {
    let sum_res = solve(1000000);
    Ok(sum_res as usize)
}

fn solve(expansion_size: u32) -> usize {
    let expansion_size = expansion_size - 1;
    let size_of_galaxy = INPUT.lines().next().unwrap().len() as i32;
    let mut galaxies = IndexSet::new();
    let mut expansion_x = IndexSet::new();
    let mut expansion_y = IndexSet::new();
    for (x, line) in INPUT.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch == '#' {
                let point = Point::new(x as i32, y as i32);
                galaxies.insert(point);
            }
        }
    }

    for xy in 0..=size_of_galaxy {
        if !galaxies.iter().any(|p| p.x == xy) {
            expansion_x.insert(xy);
        }
        if !galaxies.iter().any(|p| p.y == xy) {
            expansion_y.insert(xy);
        }
    }

    let mut expanded_galaxies = IndexSet::new();
    for galaxy in galaxies.iter() {
        let mut x = galaxy.x;
        for exp in expansion_x.iter() {
            if *exp > galaxy.x {
                break;
            } else {
                x += expansion_size as i32;
            }
        }

        let mut y = galaxy.y;
        for exp in expansion_y.iter() {
            if *exp > galaxy.y {
                break;
            } else {
                y += expansion_size as i32;
            }
        }
        expanded_galaxies.insert(Point::new(x, y));
    }

    let mut sum_res = 0;

    let mut checked = IndexSet::new();
    for x in 0..expanded_galaxies.iter().len() {
        for y in (x + 1)..expanded_galaxies.iter().len() {
            let a = expanded_galaxies.get_index(x).unwrap();
            let b = expanded_galaxies.get_index(y).unwrap();
            if checked.contains(b) {
                continue;
            }
            let d = a.manhattan_distance(b);
            sum_res += d as usize;
        }
        checked.insert(expanded_galaxies.get_index(x).unwrap());
    }
    sum_res
}
