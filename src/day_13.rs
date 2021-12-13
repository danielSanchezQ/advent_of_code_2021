use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::io::BufRead;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn fold_x(&self, coord: usize) -> Self {
        if self.x < coord {
            *self
        } else {
            Self {
                x: coord - (self.x - coord),
                y: self.y,
            }
        }
    }
    fn fold_y(&self, coord: usize) -> Self {
        if self.y < coord {
            *self
        } else {
            Self {
                x: self.x,
                y: coord - (self.y - coord),
            }
        }
    }

    fn fold(&self, by: Fold) -> Self {
        match by {
            Fold::X(coord) => self.fold_x(coord),
            Fold::Y(coord) => self.fold_y(coord),
        }
    }
}

struct Matrix(HashSet<Position>);

impl Matrix {
    fn fold_by(&self, fold: Fold) -> Self {
        Self(self.0.iter().map(|p| p.fold(fold)).collect())
    }

    fn size(&self) -> (usize, usize) {
        (
            self.0.iter().map(|p| p.y).max().unwrap(),
            self.0.iter().map(|p| p.x).max().unwrap(),
        )
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (rows, columns) = self.size();
        for row in 0..=rows {
            for column in 0..=columns {
                f.write_char(
                    self.0
                        .contains(&Position { x: column, y: row })
                        .then_some('0')
                        .unwrap_or(' '),
                )?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl FromStr for Matrix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter_map(|l| {
                    sscanf::scanf!(l.trim(), "{},{}", usize, usize)
                        .map(|(x, y): (usize, usize)| Position { x, y })
                })
                .collect(),
        ))
    }
}

#[derive(Clone, Copy, Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, value): (String, usize) =
            sscanf::scanf!(s, "fold along {}={}", String, usize).unwrap();
        match axis.as_str() {
            "x" => Ok(Self::X(value)),
            "y" => Ok(Self::Y(value)),
            _ => Err(()),
        }
    }
}

fn parse_input(mut reader: impl BufRead) -> (Matrix, Vec<Fold>) {
    let mut buff = String::new();
    reader.read_to_string(&mut buff).unwrap();
    let mut parts = buff.splitn(2, "\r\n\r\n");
    (
        parts.next().unwrap().parse().unwrap(),
        parts
            .next()
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect(),
    )
}

fn solve(matrix: Matrix, folds: Vec<Fold>) -> Matrix {
    folds
        .into_iter()
        .fold(matrix, |matrix, f| matrix.fold_by(f))
}

#[cfg(test)]
mod test {
    use crate::day_13::{parse_input, solve};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn parse_example_input() -> std::io::Result<()> {
        let reader =
            io::open_file_read(&PathBuf::from_str("./inputs/day_13_example.txt").unwrap())?;
        let (matrix, folds) = parse_input(reader);
        assert_eq!(matrix.0.len(), 18);
        assert_eq!(folds.len(), 2);
        Ok(())
    }

    #[test]
    fn example_part_1() -> std::io::Result<()> {
        let reader =
            io::open_file_read(&PathBuf::from_str("./inputs/day_13_example.txt").unwrap())?;
        let (matrix, folds) = parse_input(reader);
        assert_eq!(solve(matrix, folds).0.len(), 16);
        Ok(())
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let reader = io::open_file_read(&PathBuf::from_str("./inputs/day_13.txt").unwrap())?;
        let (matrix, folds) = parse_input(reader);
        println!(
            "Day 13 part 1 solution: {}",
            solve(matrix, folds.iter().copied().take(1).collect::<Vec<_>>())
                .0
                .len()
        );
        Ok(())
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let reader = io::open_file_read(&PathBuf::from_str("./inputs/day_13.txt").unwrap())?;
        let (matrix, folds) = parse_input(reader);
        println!("Day 13 part 1 solution: \n{}", solve(matrix, folds));
        Ok(())
    }
}
