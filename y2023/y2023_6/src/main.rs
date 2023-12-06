use anyhow::{Ok, Result};

const INPUT: &str = include_str!("../input.txt");

fn quadric_solver(time: usize, dist: usize) -> usize {
    let t = time as f64;
    let d = dist as f64;
    let tmp = f64::sqrt(t * t - 4.0 * d);
    (((t + tmp) / 2.0).floor() - ((t - tmp) / 2.0).ceil()) as usize + 1
}

fn part1() -> Option<usize> {
    let mut lines = INPUT.lines();
    let time = lines
        .next()?
        .strip_prefix("Time:")?
        .split_whitespace()
        .map(|s: &str| s.parse::<usize>().unwrap());

    let dist = lines
        .next()?
        .strip_prefix("Distance:")?
        .split_whitespace()
        .map(|s: &str| s.parse::<usize>().unwrap());
    Some(
        time.zip(dist)
            .map(|(time, dist)| quadric_solver(time, dist))
            .product(),
    )
}

fn part2() -> Option<usize> {
    let mut lines = INPUT.lines();
    let time = lines
        .next()?
        .strip_prefix("Time:")?
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();

    let dist = lines
        .next()?
        .strip_prefix("Distance:")?
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    Some(quadric_solver(time, dist))
}

fn main() -> Result<()> {
    let part1 = part1().unwrap();
    println!("Part 1: {}", part1);

    let part2 = part2().unwrap();
    println!("Part 2: {}", part2);

    Ok(())
}
