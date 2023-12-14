#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
use aoc_util::{Direction, Point};
use indexmap::IndexSet;
use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::vec;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    // let parsed = parse(INPUT)?;

    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse(input: &str) -> Result<(HashSet<Point>, HashSet<Point>)> {
    let mut immovable_rocks = HashSet::new();
    let mut rocks = HashSet::new();
    let len = input.lines().count();
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| match ch {
            '#' => {
                immovable_rocks.insert(Point::new(col as i32, (len - row) as i32));
            }
            'O' => {
                rocks.insert(Point::new(col as i32, (len - row) as i32));
            }
            _ => {}
        });
    });
    Ok((immovable_rocks, rocks))
}

fn part1() -> Result<usize> {
    let (immovable_rocks, initial_rocks) = parse(INPUT)?;
    let mut rocks = initial_rocks.iter().cloned().collect_vec();
    let max_width = INPUT.lines().next().unwrap().chars().count() as i32;
    let max_height = INPUT.lines().count() as i32;

    move_dir(
        &mut rocks,
        &immovable_rocks,
        Direction::North,
        max_width,
        max_height,
    );

    let support = calculate_north_support(&rocks);

    Ok(support)
}

fn part2() -> Result<usize> {
    let (immovable_rocks, initial_rocks) = parse(INPUT)?;
    let mut rocks = initial_rocks.iter().cloned().collect_vec();
    let max_width = INPUT.lines().next().unwrap().chars().count() as i32;
    let max_height = INPUT.lines().count() as i32;

    let mut seen = HashSet::new();
    for dir in Direction::cardinals().iter().cycle() {
        move_dir(&mut rocks, &immovable_rocks, *dir, max_width, max_height);
        let hash = calculate_rock_hash(&rocks);
        if seen.contains(&hash) {
            println!("hash: {}", hash);
        } else {
            seen.insert(hash);
        }
    }

    let support = calculate_north_support(&rocks);

    Ok(support)
}

// fn move_dir(
//     rocks: &mut Vec<Point>,
//     immovable_rocks: &HashSet<Point>,
//     dir: Direction,
//     max_width: i32,
//     max_height: i32,
// ) {
//     loop {
//         let mut updated = false;
//         for idx in 0..rocks.len() {
//             let next = rocks[idx] + dir;
//             if !immovable_rocks.contains(&next)
//                 && !rocks.contains(&next)
//                 && is_inbound(&next, max_height, max_width)
//             {
//                 rocks[idx] = next;
//                 updated = true;
//             }
//         }
//         if !updated {
//             break;
//         }
//     }
// }

fn move_dir(
    rocks: &mut Vec<Point>,
    immovable_rocks: &HashSet<Point>,
    dir: Direction,
    max_width: i32,
    max_height: i32,
) {
    loop {
        let mut updated = false;
        for idx in 0..rocks.len() {
            let next = rocks[idx] + dir;
            if !immovable_rocks.contains(&next)
                && !rocks.contains(&next)
                && is_inbound(&next, max_height, max_width)
            {
                rocks[idx] = next;
                updated = true;
            }
        }
        if !updated {
            break;
        }
    }
}

fn calculate_rock_hash(rocks: &Vec<Point>) -> u64 {
    let mut s = DefaultHasher::new();
    for p in rocks.iter() {
        p.hash(&mut s);
    }
    s.finish()
}

fn is_inbound(point: &Point, max_height: i32, max_width: i32) -> bool {
    point.x >= 0 && point.x <= max_width && point.y >= 0 && point.y <= max_height
}

fn calculate_north_support(rocks: &Vec<Point>) -> usize {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for point in rocks.iter() {
        *map.entry(point.y).or_default() += 1;
    }
    let res = map.iter().map(|(key, freq)| *key as usize * *freq).sum();
    res
}
