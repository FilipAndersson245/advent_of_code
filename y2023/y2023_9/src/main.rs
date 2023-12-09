#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
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
    let inputs = INPUT
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|ch| ch.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let res: i64 = inputs.iter().map(extrapolate_value_part1).sum();
    Ok(res as usize)
}

fn part2() -> Result<usize> {
    let inputs = INPUT
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|ch| ch.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let res: i64 = inputs.iter().map(extrapolate_value_part2).sum();
    Ok(res as usize)
}

fn extrapolate_value_part1(x: &Vec<i64>) -> i64 {
    if is_all_zeroes(x) {
        return 0;
    }

    let value_to_append = extrapolate_value_part1(&generate_differences(x));

    x.last().unwrap() + value_to_append
}

fn extrapolate_value_part2(x: &Vec<i64>) -> i64 {
    if is_all_zeroes(x) {
        return 0;
    }

    let values_to_prepend = extrapolate_value_part2(&generate_differences(x));

    x.first().unwrap() - values_to_prepend
}

fn generate_differences(x: &Vec<i64>) -> Vec<i64> {
    x.windows(2).map(|x| x[1] - x[0]).collect()
}

fn is_all_zeroes(x: &Vec<i64>) -> bool {
    x.iter().all(|n| *n == 0)
}
