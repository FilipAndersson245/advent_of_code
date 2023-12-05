#![allow(unused_imports)]
#![allow(dead_code)]

use std::str::FromStr;

use anyhow::{Ok, Result};
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Hash)]
struct Mapping {
    dest_range: usize,
    source_range: usize,
    range_length: usize,
}

struct MappingFoo {
    dest_name: String,
    source_name: String,
    mappings: Vec<Mapping>,
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(' ');
        let dest_range = parts.next().unwrap().parse::<usize>()?;
        let source_range = parts.next().unwrap().parse::<usize>()?;
        let range_length = parts.next().unwrap().parse::<usize>()?;

        Ok(Mapping {
            dest_range,
            source_range,
            range_length,
        })
    }
}

impl From<(String, String, Vec<Mapping>)> for MappingFoo {
    fn from((dest_name, source_name, mappings): (String, String, Vec<Mapping>)) -> Self {
        Self {
            dest_name,
            source_name,
            mappings,
        }
    }
}

// TODO maybe cache?
impl MappingFoo {
    fn map(&self, input: usize) -> usize {
        let mut output = input;
        for mapping in &self.mappings {
            if input >= mapping.source_range && input < mapping.source_range + mapping.range_length
            {
                output = mapping.dest_range + (input - mapping.source_range);
                break;
            }
        }
        output
    }
}

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1() -> Result<usize> {
    let input = INPUT;
    let initial_seeds = get_initial_seeds(input);
    let mappings = get_mappings(input)?
        .iter()
        .map(|mapping| MappingFoo::from(mapping.clone()))
        .collect_vec();

    let mut total = usize::MAX;
    for seed in &initial_seeds {
        let mut seed = *seed;
        for mapping in &mappings {
            seed = mapping.map(seed);
        }

        if seed < total {
            total = seed;
        }
    }
    Ok(total)
}

use rayon::prelude::*;

fn part2() -> Result<usize> {
    let input = INPUT;
    let initial_seeds = get_initial_seeds(input);

    let mappings = get_mappings(input)?
        .iter()
        .map(|mapping| MappingFoo::from(mapping.clone()))
        .collect_vec();

    let mut total = usize::MAX;

    // for seed in &seed_vec {
    //     let mut seed = *seed;
    //     for mapping in &mappings {
    //         seed = mapping.map(seed);
    //     }

    //     if seed < total {
    //         total = seed;
    //     }
    // }

    initial_seeds.chunks(2).for_each(|chunk| {
        let start = chunk[0];
        let end = chunk[0] + chunk[1];
        let smallest = (start..end)
            .into_par_iter()
            .map(|seed| {
                let mut seed = seed;
                for mapping in &mappings {
                    seed = mapping.map(seed);
                }
                seed
            })
            .min()
            .unwrap();
        if smallest < total {
            total = smallest;
        }
    });

    assert_eq!(52510809, total);
    Ok(total)
}

aoc_util::parsing::capture_regex!(from_and_to = r"([a-zA-Z]+)-to-([a-zA-Z]+)", String, String);

fn get_mappings(input: &str) -> Result<Vec<(String, String, Vec<Mapping>)>> {
    let data = input
        .split("\n\n")
        .skip(1)
        .map(|lines| {
            let mut line = lines.lines();
            let (dest_name, source_name) = line.next().map(from_and_to).unwrap().unwrap();
            let mappings = line
                .map(|line| line.parse::<Mapping>().unwrap())
                .collect_vec();
            (dest_name, source_name, mappings)
        })
        .collect_vec();
    Ok(data)
}

fn get_initial_seeds(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect_vec()
}
