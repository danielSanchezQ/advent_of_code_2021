use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, IndexMut};
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

    fn get_surrounds(&self, row: usize, column: usize) -> impl Iterator<Item = u8> + '_ {
        SurroundsIterator::new(self, row, column)
    }

    fn inc(&mut self, value: u8) {
        self.0.iter_mut().for_each(|v| {
            v.iter_mut().for_each(|mut e| {
                *e += value;
            })
        })
    }

    fn flash_at(&mut self, row: isize, column: isize) -> usize {
        self.get_mut(row, column).iter_mut().for_each(|e| **e += 1);
        if Some(10) == self.get(row, column) {
            CHECKS
                .iter()
                .map(|(r, c)| self.flash_at(row + r, column + c))
                .sum()
        } else {
            0
        }
    }

    fn flash(&mut self) -> usize {
        let mut ret = 0;
        for row in 0..self.0.len() {
            for column in 0..self.0[0].len() {
                if self.0[row][column] > 9 {
                    ret += self.flash_at(row as isize, column as isize);
                }
            }
        }
        ret
    }

    fn count(&self) -> usize {
        self.0
            .iter()
            .flat_map(|v| v.iter())
            .filter(|n| **n > 9)
            .count()
    }

    fn reset(&mut self) {
        self.0.iter_mut().flat_map(|v| v.iter_mut()).for_each(|v| {
            if *v > 9u8 {
                *v = 0;
            }
        });
    }

    fn any_flash(&self) -> bool {
        self.0.iter().flat_map(|v| v.iter()).any(|n| *n > 9)
    }

    fn flash_count_reset(&mut self) -> usize {
        self.flash();
        let mut ret = 0;
        while self.any_flash() {
            ret += self.count();
            self.reset();
        }
        ret
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
    (0..steps)
        .map(|_| {
            matrix.inc(1);
            println!("{}", matrix);
            let ret = matrix.flash_count_reset();
            println!("{}", matrix);
            ret
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day_11::{simulate, Matrix};

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
        assert_eq!(simulate(matrix, 2), 1656);
    }
}
