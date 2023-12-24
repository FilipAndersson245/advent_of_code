use anyhow::Result;

const INPUT: &str = include_str!("../input.txt");

pub struct Grid {
    data: Box<[u8]>,
    offset: usize,
}
impl Grid {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let line_len = lines.peek().map_or(0, |l| l.len());
        Self {
            data: lines.flat_map(str::as_bytes).copied().collect::<Box<_>>(),
            offset: line_len,
        }
    }
    const fn next_pos(&self, p: usize, dir: u8) -> Option<usize> {
        // 0 = up, 1 = right, 2 = down, 3 = left
        Some(match dir {
            0 if p >= self.offset => p - self.offset,
            1 if (p + 1) % self.offset != 0 => p + 1,
            2 if p < self.data.len() - self.offset => p + self.offset,
            3 if p % self.offset != 0 => p - 1,
            _ => return None,
        })
    }
    const fn find_s(&self) -> usize {
        self.data.len() / 2
    }
}

pub fn parse_grid(s: &str) -> (Grid, usize) {
    let g = Grid::from_str(s);
    let start = g.find_s();
    (g, start)
}

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("Part 1: {}", part1);

    let part2 = part2()?;
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1() -> Result<usize> {
    let (g, start) = parse_grid(INPUT);
    let steps = 64;
    let odd_bit = steps & 1;
    let mut checked = g.data.iter().map(|c| *c == b'#').collect::<Vec<_>>();
    let (mut set, mut set_next) = (Vec::new(), Vec::new());
    let mut reach = 0;
    checked[start] = true;
    set.push(start);
    for st in 0..=steps {
        for &p in &set {
            if st & 1 == odd_bit {
                reach += 1;
            }
            for dir in 0..4 {
                if let Some(np) = g.next_pos(p, dir) {
                    if !checked[np] {
                        checked[np] = true;
                        set_next.push(np);
                    }
                }
            }
        }
        set.clear();
        std::mem::swap(&mut set, &mut set_next);
    }
    Ok(reach)
}

fn part2() -> Result<usize> {
    Ok(0)
}
