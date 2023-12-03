use anyhow::Result;
use aoc_util::{HashMap, HashSet, Point};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone)]
struct Part {
    value: usize,
    pos: Vec<Point>,
}

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1() -> Result<usize> {
    let mut symbols = HashMap::default();
    for (y, line) in INPUT.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => continue,
                digit if digit.is_ascii_digit() => continue,
                symbol => {
                    let point = Point::new(x as i32, y as i32);
                    symbols.insert(point, symbol);
                }
            }
        }
    }

    let parts = get_parts();

    let mut valid_parts = Vec::new();
    for part in parts.iter() {
        'part: for part_pos in part.pos.iter() {
            for neighbor in part_pos.eightway() {
                if symbols.get(&neighbor).is_some() {
                    valid_parts.push(part);
                    break 'part;
                }
            }
        }
    }

    let res: usize = valid_parts.iter().map(|part| part.value).sum();
    Ok(res)
}

fn part2() -> Result<usize> {
    let mut gears = HashSet::default();
    for (y, line) in INPUT.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '*' => {
                    let point = Point::new(x as i32, y as i32);
                    gears.insert(point);
                }
                _ => continue,
            }
        }
    }

    let parts = get_parts();

    let mut parts_map = HashMap::default();
    for (i, part) in parts.iter().enumerate() {
        for pos in part.pos.iter() {
            parts_map.insert(pos, i);
        }
    }

    let mut res = 0;
    for gear in gears.iter() {
        let mut neighbor_parts = HashSet::default();
        for neighbor in gear.eightway().iter() {
            if let Some(part_nr) = parts_map.get(&neighbor) {
                neighbor_parts.insert(*part_nr);
            }
        }

        if neighbor_parts.len() == 2 {
            res += neighbor_parts
                .iter()
                .map(|key| parts[*key].value)
                .product::<usize>();
        }
    }

    let mut valid_parts = Vec::new();
    for part in parts.iter() {
        'part: for part_pos in part.pos.iter() {
            for neighbor in part_pos.eightway() {
                if gears.get(&neighbor).is_some() {
                    valid_parts.push(part);
                    break 'part;
                }
            }
        }
    }

    Ok(res)
}

fn get_parts() -> Vec<Part> {
    let mut parts = Vec::new();
    for (y, line) in INPUT.lines().enumerate() {
        let mut iter = line.char_indices();
        while let Some((x, c)) = iter.next() {
            if c == '.' {
                continue;
            } else if c.is_ascii_digit() {
                let mut pos = Vec::new();
                pos.push(Point::new(x as i32, y as i32));

                let mut part_value = c.to_digit(10).unwrap() as usize;

                for (x, c) in iter.by_ref() {
                    if c == ' ' {
                        continue;
                    }
                    if c.is_ascii_digit() {
                        part_value *= 10;
                        part_value += c.to_digit(10).unwrap() as usize;
                        pos.push(Point::new(x as i32, y as i32));
                    } else {
                        break;
                    }
                }
                parts.push(Part {
                    value: part_value,
                    pos,
                });
            }
        }
    }
    parts
}
