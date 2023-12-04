use anyhow::Result;
use aoc_util::HashSet;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

use cached::proc_macro::cached;

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1() -> Result<usize> {
    let parsed_input = parse_input();

    let mut total_score = 0;
    for (_, (winning_numb, your_numb)) in parsed_input {
        let mut score = 0;
        for your_numb in your_numb {
            if winning_numb.contains(&your_numb) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        total_score += score;
    }
    Ok(total_score)
}

fn part2() -> Result<usize> {
    let parsed_input = parse_input();

    let mut total_score = 0;
    for (idx, (winning_numb, your_numb)) in parsed_input.iter() {
        total_score += rec_solver(&parsed_input, &winning_numb, &your_numb, *idx) + 1;
    }
    Ok(total_score)
}

use cached::SizedCache;

#[cached(
    type = "SizedCache<usize, usize>",
    create = "{ SizedCache::with_size(150) }",
    convert = r#"{ idx }"#
)]
fn rec_solver(
    all_scratches: &Vec<(usize, (HashSet<usize>, HashSet<usize>))>,
    winning_numb: &HashSet<usize>,
    your_numb: &HashSet<usize>,
    idx: usize,
) -> usize {
    let mut matches = 0;

    for your_numb in your_numb {
        if winning_numb.contains(&your_numb) {
            matches += 1;
        }
    }
    if matches == 0 {
        return 0;
    }

    for i in 1..=matches {
        let (index, (winning_numb, your_numb)) = all_scratches.get(idx + i).unwrap();
        matches += rec_solver(all_scratches, &winning_numb, &your_numb, *index)
    }

    return matches;
}

fn parse_input() -> Vec<(usize, (HashSet<usize>, HashSet<usize>))> {
    let parsed_input = INPUT
        .lines()
        .map(|line| {
            let s = line.split(':').last().unwrap().trim();
            let mut iter = s.split('|');
            let winning_numb = iter.next().unwrap().trim();
            let your_numb = iter.next().unwrap().trim();
            let winning_numb: HashSet<usize> = winning_numb
                .split(' ')
                .filter(|s| s != &"")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let your_numb: HashSet<usize> = your_numb
                .split(' ')
                .filter(|s| s != &"")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (winning_numb, your_numb)
        })
        .enumerate()
        .collect_vec();
    parsed_input
}
