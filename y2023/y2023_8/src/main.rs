#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

use aoc_util::parsing::{capture_regex, capture_regex_line};
use std::collections::{HashMap, HashSet};

use cached::proc_macro::cached;
use cached::SizedCache;

fn main() -> Result<()> {
    // let part1 = part1()?;
    // println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

capture_regex_line!(
    parse_map = r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)",
    String,
    String,
    String
);

fn part1() -> Result<usize> {
    let mut input = INPUT.split("\n\n");
    let walking_dir = input.next().unwrap().chars().collect_vec();
    let map = input.next().map(|s| parse_map(s).unwrap()).unwrap();
    let map: HashMap<String, (String, String)> =
        map.into_iter().map(|(k, l, r)| (k, (l, r))).collect();

    let mut curr_pos = "AAA".to_string();
    let len = walking_dir.len();
    let mut i = 0;
    let mut s = 0;
    loop {
        let ch = walking_dir[i];
        i = (i + 1) % len;
        curr_pos = match ch {
            'L' => map.get(&curr_pos).unwrap().0.clone(),
            'R' => map.get(&curr_pos).unwrap().1.clone(),
            _ => {
                panic!()
            }
        };
        s += 1;
        println!("{}", curr_pos);
        if curr_pos == "ZZZ" {
            break;
        }
    }
    Ok(s)
}

fn part2() -> Result<usize> {
    let mut input = INPUT.split("\n\n");
    let walking_dir = input.next().unwrap().chars().collect_vec();
    let map = input.next().map(|s| parse_map(s).unwrap()).unwrap();

    let start_pos: HashSet<_> = map
        .iter()
        .filter_map(|(k, _, _)| {
            if k.ends_with('A') {
                Some(k.clone())
            } else {
                None
            }
        })
        .collect();

    let end_pos: HashSet<_> = map
        .iter()
        .filter_map(|(_, a, b)| {
            if a.ends_with('Z') {
                Some(a.clone())
            } else if b.ends_with('Z') {
                Some(b.clone())
            } else {
                None
            }
        })
        .collect();

    println!("s: {:?}", start_pos);
    println!("e: {:?}", end_pos);
    println!("");

    let map: HashMap<String, (String, String)> =
        map.into_iter().map(|(k, l, r)| (k, (l, r))).collect();

    // let len = walking_dir.len();
    let curr_pos = start_pos.iter().cloned().collect_vec();
    let indexes = vec![0; curr_pos.len()];
    let mut steps = vec![0; curr_pos.len()];

    for i in 0..curr_pos.len() {
        let (_, s, _) = get_path(indexes[i], &curr_pos[i], &map, &walking_dir);

        steps[i] = s;
    }
    use num::integer::lcm;

    // lcm(x, y)
    println!("{:?}", steps);
    let a = steps.into_iter().reduce(|a, b| lcm(a, b)).unwrap();

    Ok(a)
}

fn get_path(
    i: usize,
    node: &str,
    map: &HashMap<String, (String, String)>,
    walking_dir: &Vec<char>,
) -> (usize, usize, String) {
    let len = walking_dir.len();
    let mut i = i;
    let mut s = 0;
    let mut pos = node.to_string();
    loop {
        let ch = walking_dir[i];
        i = (i + 1) % len;

        pos = match ch {
            'L' => map.get(&pos).unwrap().0.clone(),
            'R' => map.get(&pos).unwrap().1.clone(),
            _ => {
                panic!()
            }
        };

        s += 1;
        if pos.ends_with('Z') {
            break;
        }
    }
    println!("{} {} {}", i, s, pos);
    (i, s, pos)
}

// LLR

// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)
