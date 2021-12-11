use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use std::str::FromStr;

static CHECKS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

#[derive(Debug, Clone)]
struct Matrix(Vec<Vec<u8>>);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for l in &self.0 {
            writeln!(f, "{:?}", l)?;
        }
        Ok(())
    }
}

impl Matrix {
    fn get(&self, row: isize, column: isize) -> Option<u8> {
        if [
            row < 0,
            column < 0,
            row as usize >= self.0.len(),
            column as usize >= self.0[0].len(),
        ]
        .iter()
        .any(|b| *b)
        {
            return None;
        }
        Some(self.0[row as usize][column as usize])
    }

    fn get_mut(&mut self, row: isize, column: isize) -> Option<&mut u8> {
        if [
            row < 0,
            column < 0,
            row as usize >= self.0.len(),
            column as usize >= self.0[0].len(),
        ]
        .iter()
        .any(|b| *b)
        {
            return None;
        }
        Some(&mut self.0[row as usize][column as usize])
    }

    fn inc(&mut self, value: u8) {
        self.0.iter_mut().for_each(|v| {
            v.iter_mut().for_each(|e| {
                *e += value;
            })
        })
    }

    fn flash_at(&mut self, row: isize, column: isize, inc: u8) -> usize {
        if let Some(n) = self.get_mut(row, column) {
            match n {
                0 => 0,
                i if *i > 9u8 => {
                    *i = 0;
                    1usize
                        + CHECKS
                            .iter()
                            .map(|(r, c)| self.flash_at(row + r, column + c, 1))
                            .sum::<usize>()
                }
                i => {
                    *i += inc;
                    0
                }
            }
        } else {
            0
        }
    }

    fn flash(&mut self) -> usize {
        let mut ret = 0;
        for row in 0..self.0.len() as isize {
            for column in 0..self.0[0].len() as isize {
                ret += self.flash_at(row, column, 0);
            }
        }
        ret
    }

    fn any_flash(&self) -> bool {
        self.0.iter().flat_map(|v| v.iter()).any(|n| *n > 9)
    }

    fn flash_count_reset(&mut self) -> usize {
        let mut ret = 0;
        while self.any_flash() {
            ret += self.flash();
            // self.reset();
        }
        ret
    }

    fn step(&mut self) -> usize {
        self.inc(1);
        self.flash_count_reset()
    }

    fn is_zero(&self) -> bool {
        self.0.iter().flat_map(|v| v.iter()).all(|n| *n == 0)
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        assert!(!self.0.is_empty());
        assert!(!rhs.0.is_empty());
        assert_eq!(self.0.len(), rhs.0.len());
        assert_eq!(self.0[0].len(), rhs.0[0].len());

        for row in 0..self.0.len() {
            for column in 0..self.0[0].len() {
                self.0[row][column] += rhs.0[row][column];
            }
        }
    }
}

struct SurroundsIterator<'matrix> {
    matrix: &'matrix Matrix,
    checks: Box<dyn Iterator<Item = &'static (isize, isize)>>,
    row: usize,
    column: usize,
}

impl<'matrix> SurroundsIterator<'matrix> {
    fn new(heatmap: &'matrix Matrix, row: usize, column: usize) -> Self {
        Self {
            matrix: heatmap,
            checks: Box::new(CHECKS.iter()),
            row,
            column,
        }
    }
}

impl<'matrix> Iterator for SurroundsIterator<'matrix> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.checks.next() {
                None => return None,
                Some(&(row, column)) => {
                    if let Some(n) = self
                        .matrix
                        .get(self.row as isize + row, self.column as isize + column)
                    {
                        return Some(n);
                    }
                }
            }
        }
    }
}

impl FromStr for Matrix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
                .collect(),
        ))
    }
}

fn simulate(mut matrix: Matrix, steps: usize) -> usize {
    (0..steps).map(|_| matrix.step()).sum()
}

fn solve_part_1(matrix: Matrix) -> usize {
    simulate(matrix, 100)
}

fn solve_part_2(mut matrix: Matrix) -> usize {
    for i in 1.. {
        matrix.step();
        if matrix.is_zero() {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::day_11::{solve_part_1, solve_part_2, Matrix};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn example_part_1() {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let matrix: Matrix = input.parse().unwrap();
        assert_eq!(solve_part_1(matrix), 1656);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let matrix: Matrix =
            io::read_object_from_file(&PathBuf::from_str("./inputs/day_11.txt").unwrap())?;
        println!("Day 11 part 1 solution: {}", solve_part_1(matrix));
        Ok(())
    }

    #[test]
    fn example_part_2() {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let matrix: Matrix = input.parse().unwrap();
        assert_eq!(solve_part_2(matrix), 195);
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let matrix: Matrix =
            io::read_object_from_file(&PathBuf::from_str("./inputs/day_11.txt").unwrap())?;
        println!("Day 11 part 2 solution: {}", solve_part_2(matrix));
        Ok(())
    }
}
