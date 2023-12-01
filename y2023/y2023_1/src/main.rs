use anyhow::Result;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1() -> Result<usize> {
    let mut all_lines = 0;
    for line in INPUT.lines() {
        let mut first = ' ';
        let mut last = ' ';
        for ch in line.chars() {
            if ch.is_ascii_digit() && first == ' ' {
                first = ch;
            } else if ch.is_ascii_digit() {
                last = ch;
            }
        }
        let data;
        if last == ' ' {
            data = format!("{}{}", first, first).parse::<usize>()?;
        } else {
            data = format!("{}{}", first, last).parse::<usize>()?;
        }

        all_lines += data;
    }
    Ok(all_lines)
}

fn part2() -> Result<u32> {
    let mut total = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in INPUT.lines() {
        let mut digits = vec![];
        let chars = line.chars().collect::<Vec<_>>();
        for (i, c) in chars.iter().enumerate() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d);
                continue;
            }
            let s = String::from_iter(&chars[i..chars.len()]);
            for (i, n) in numbers.iter().enumerate() {
                if s.starts_with(n) {
                    digits.push(i as u32 + 1);
                }
            }
        }
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        total += number.parse::<u32>().unwrap();
    }
    Ok(total)
}
