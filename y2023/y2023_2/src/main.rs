use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
struct Cube {
    numb: usize,
    color: Color,
}
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Cube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.trim().split_whitespace();
        let numb = parts.next().unwrap().parse()?;
        let color = match parts.next().unwrap() {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => unreachable!(),
        };
        Ok(Cube { numb, color })
    }
}

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1() -> Result<usize> {
    let games = parse_game();

    let mut sum_valid_game_ids = 0;
    for (i, sets) in games.iter().enumerate() {
        let game_id = i + 1;
        let mut is_valid_game = true;

        for set in sets {
            for Cube { numb, color } in set {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                match color {
                    Color::Red => red += numb,
                    Color::Blue => blue += numb,
                    Color::Green => green += numb,
                }

                if red > 12 || blue > 14 || green > 13 {
                    is_valid_game = false;
                }
            }
        }
        if is_valid_game {
            sum_valid_game_ids += game_id;
        }
    }
    Ok(sum_valid_game_ids)
}

fn parse_game() -> Vec<Vec<Vec<Cube>>> {
    let games = INPUT
        .lines()
        .map(|l| {
            l.split(';')
                .map(|sets| {
                    sets.split(",")
                        .map(|cube_str| cube_str.parse().unwrap())
                        .collect::<Vec<Cube>>()
                })
                .collect::<Vec<Vec<Cube>>>()
        })
        .collect::<Vec<Vec<Vec<Cube>>>>();
    games
}

fn part2() -> Result<usize> {
    let games = parse_game();
    let mut sum = 0;
    for (i, sets) in games.iter().enumerate() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in sets {
            for Cube { numb, color } in set {
                match color {
                    Color::Red if *numb > red => red = *numb,
                    Color::Blue if *numb > blue => blue = *numb,
                    Color::Green if *numb > green => green = *numb,
                    _ => {}
                }
            }
        }
        sum += (red * blue * green);
    }
    Ok(sum)
}
