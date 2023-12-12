#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn parse(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let pattern = parts.next().unwrap();
            let counts = parts
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            (pattern, counts)
        })
        .collect()
}

fn count_arrangements(line: &str, counts: &[usize]) -> usize {
    let line = line.as_bytes();
    let n = line.len();
    let m = counts.len();
    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];

    dp[n][m][0] = 1;
    dp[n][m - 1][counts[m - 1]] = 1;

    for pos in (0..n).rev() {
        for (group, &max_count) in counts.iter().enumerate() {
            for count in 0..=max_count {
                for &c in &[b'.', b'#'] {
                    if line[pos] == c || line[pos] == b'?' {
                        if c == b'.' && count == 0 {
                            dp[pos][group][count] += dp[pos + 1][group][0];
                        } else if c == b'.' && group < m && counts[group] == count {
                            dp[pos][group][count] += dp[pos + 1][group + 1][0];
                        } else if c == b'#' {
                            dp[pos][group][count] += dp[pos + 1][group][count + 1];
                        }
                    }
                }
            }
        }
        if matches!(line[pos], b'.' | b'?') {
            dp[pos][m][0] += dp[pos + 1][m][0];
        }
    }

    dp[0][0][0]
}

fn main() -> Result<()> {
    let parsed = parse(INPUT);
    let part1 = part1(&parsed)?;
    println!("Part 1: {}", part1);

    let part2 = part2(&parsed)?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1(input: &[(&str, Vec<usize>)]) -> Result<usize> {
    Ok(input
        .iter()
        .map(|(pattern, counts)| count_arrangements(pattern, counts))
        .sum::<usize>())
}

fn part2(input: &[(&str, Vec<usize>)]) -> Result<usize> {
    Ok(input
        .iter()
        .map(|(pattern, counts)| {
            let pattern = [*pattern; 5].join("?");
            let counts = counts.repeat(5);
            count_arrangements(&pattern, &counts)
        })
        .sum::<usize>())
}
