use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
struct Horizontal {
    x: RangeInclusive<usize>,
    y: usize,
}

#[derive(Debug)]
struct Vertical {
    x: usize,
    y: RangeInclusive<usize>,
}

#[derive(Debug)]
struct Free {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}

#[derive(Debug)]
enum Line {
    Horizontal(Horizontal),
    Vertical(Vertical),
    Free(Free),
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x1, y1, x2, y2) =
            sscanf::scanf!(s, "{},{} -> {},{}", usize, usize, usize, usize).unwrap();
        Ok(if x1 == x2 {
            Self::Vertical(Vertical {
                x: x1,
                y: (y1.min(y2)..=y2.max(y1)),
            })
        } else if y1 == y2 {
            Self::Horizontal(Horizontal {
                x: (x1.min(x2)..=x2.max(x1)),
                y: y1,
            })
        } else {
            Self::Free(Free {
                x: (x1..=x2),
                y: (y1..=y2),
            })
        })
    }
}

fn reverse_inverse_range(range: &RangeInclusive<usize>) -> Box<dyn Iterator<Item = usize>> {
    if range.start() < range.end() {
        Box::new(range.clone().into_iter())
    } else {
        Box::new((*range.end()..=*range.start()).rev())
    }
}

fn solve(lines: &[Line], free: bool, threshold: usize) -> usize {
    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
    for line in lines.iter() {
        match line {
            Line::Horizontal(Horizontal { x, y }) => {
                for next_x in x.clone() {
                    *grid.entry((next_x, *y)).or_default() += 1;
                }
            }
            Line::Vertical(Vertical { x, y }) => {
                for next_y in y.clone() {
                    *grid.entry((*x, next_y)).or_default() += 1;
                }
            }
            Line::Free(Free { x, y }) if free => {
                for (next_x, next_y) in reverse_inverse_range(x).zip(reverse_inverse_range(y)) {
                    *grid.entry((next_x, next_y)).or_default() += 1;
                }
            }
            _ => {}
        }
    }
    grid.values().filter(|v| **v >= threshold).count()
}

fn solve_part_1(lines: &[Line]) -> usize {
    solve(lines, false, 2)
}

fn solve_part_2(lines: &[Line]) -> usize {
    solve(lines, true, 2)
}

#[cfg(test)]
mod test {
    use crate::day_05::{solve_part_1, solve_part_2, Line};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn example_part_1() -> std::io::Result<()> {
        let lines: Vec<Line> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_05_example.txt").unwrap())?;
        assert_eq!(solve_part_1(&lines), 5);
        Ok(())
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let lines: Vec<Line> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_05.txt").unwrap())?;
        println!("Day 5 part 1 solution: {}", solve_part_1(&lines));
        Ok(())
    }

    #[test]
    fn example_part_2() -> std::io::Result<()> {
        let lines: Vec<Line> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_05_example.txt").unwrap())?;
        assert_eq!(solve_part_2(&lines), 12);
        Ok(())
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let lines: Vec<Line> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_05.txt").unwrap())?;
        println!("Day 5 part 2 solution: {}", solve_part_2(&lines));
        Ok(())
    }
}
