#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::{Ok, Result};
pub use indexmap::IndexMap;
use itertools::Itertools;
use linked_hash_map::LinkedHashMap;

const INPUT: &str = include_str!("../input.txt");

fn hash(text: &str) -> usize {
    let mut value = 0;
    for c in text.chars() {
        value = (value + (c as usize)) * 17 % 256;
    }
    value
}

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1() -> Result<usize> {
    let input: Vec<&str> = INPUT.split(',').collect();

    Ok(input.iter().map(|&x| hash(x)).sum::<usize>())
}

fn part2() -> Result<usize> {
    let input: Vec<&str> = INPUT.split(',').collect();

    let mut boxes = vec![LinkedHashMap::<&str, u8>::new(); 256];

    for item in input {
        let mut step = item.split(|c| c == '=' || c == '-');

        let label = step.next().unwrap();
        let focal_length = step.next().unwrap();

        let box_id = hash(label);

        if focal_length.is_empty() {
            boxes[box_id].remove(label);
        } else {
            *boxes[box_id].entry(label).or_insert(0) = focal_length.parse().unwrap();
        }
    }

    let mut res = 0;

    for (b, _box) in boxes.iter().enumerate() {
        for l in 0.._box.len() {
            let label = _box.keys().nth(l).unwrap();
            res += (b + 1) * (l + 1) * boxes[b][label] as usize;
        }
    }

    Ok(res)
}
