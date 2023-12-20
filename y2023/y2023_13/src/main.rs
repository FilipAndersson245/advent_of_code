#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
use grid::Grid;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn to_char_grid(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

fn transpose(value: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..value[0].len())
        .map(|i| value.iter().map(|col| col[i]).collect())
        .collect()
}

pub fn solve() -> (usize, usize) {
    let input: Vec<&str> = INPUT.split("\n\n").collect();

    let result1 = input
        .iter()
        .map(|&p| calculate(to_char_grid(p)))
        .sum::<usize>();
    let result2 = input
        .iter()
        .map(|&p| fix_smudge(to_char_grid(p)))
        .sum::<usize>();

    println!("13\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn fix_smudge(input: Vec<Vec<char>>) -> usize {
    let mirror_row = find_mirror(&input, None);
    let mirror_col = find_mirror(&transpose(&input), None);

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let test = check_smudge(&input, row, col);

            if let Some(r) = find_mirror(&test, mirror_row) {
                return 100 * r;
            }

            if let Some(c) = find_mirror(&transpose(&input), mirror_col) {
                return c;
            }
        }
    }

    panic!("No solution found");
}

fn check_smudge(input: &[Vec<char>], row: usize, col: usize) -> Vec<Vec<char>> {
    let mut result = input.to_vec();

    result[row][col] = match result[row][col] {
        '#' => '.',
        '.' => '#',
        _ => panic!("Invalid tile"),
    };

    result
}

fn calculate(rows: Vec<Vec<char>>) -> usize {
    100 * find_mirror(&rows, None).unwrap_or(0) + find_mirror(&transpose(&rows), None).unwrap_or(0)
}

fn find_mirror(input: &[Vec<char>], skip: Option<usize>) -> Option<usize> {
    (1..input.len())
        .filter(|&i| skip.is_none() || i != skip.unwrap())
        .find(|&i| is_mirrored(input, i))
}

fn is_mirrored(input: &[Vec<char>], row: usize) -> bool {
    let mut upper = input.iter().take(row).rev().collect::<Vec<_>>();
    let mut lower = input.iter().skip(row).collect::<Vec<_>>();

    let length = upper.len().min(lower.len());

    upper = upper.iter().take(length).copied().collect::<Vec<_>>();
    lower = lower.iter().take(length).copied().collect::<Vec<_>>();

    upper == lower
}

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1() -> Result<usize> {
    let input: Vec<&str> = INPUT.split("\n\n").collect();

    let res = input
        .iter()
        .map(|&p| calculate(to_char_grid(p)))
        .sum::<usize>();
    Ok(res)
}

fn part2() -> Result<usize> {
    let input: Vec<&str> = INPUT.split("\n\n").collect();
    let res = input
        .iter()
        .map(|&p| fix_smudge(to_char_grid(p)))
        .sum::<usize>();
    Ok(res)
}
