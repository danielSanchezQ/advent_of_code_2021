use std::collections::HashSet;
use std::str::FromStr;

static CHECKS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

struct HeatMap(Vec<Vec<u8>>);

impl FromStr for HeatMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|l| {
                    l.trim()
                        .chars()
                        .map(|c| c.to_string().parse().unwrap())
                        .collect::<Vec<u8>>()
                })
                .collect(),
        ))
    }
}

struct PointSurroundsIterator<'heatmap> {
    heatmap: &'heatmap HeatMap,
    checks: Box<dyn Iterator<Item = &'static (isize, isize)>>,
    row: usize,
    column: usize,
}

impl<'heatmap> PointSurroundsIterator<'heatmap> {
    fn new(heatmap: &'heatmap HeatMap, row: usize, column: usize) -> Self {
        Self {
            heatmap,
            checks: Box::new(CHECKS.iter()),
            row,
            column,
        }
    }
}

impl<'heatmap> Iterator for PointSurroundsIterator<'heatmap> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.checks.next() {
                None => return None,
                Some(&(row, column)) => {
                    if let Some(n) = self
                        .heatmap
                        .get(self.row as isize + row, self.column as isize + column)
                    {
                        return Some(n);
                    }
                }
            }
        }
    }
}

impl HeatMap {
    fn get_surrounds(&self, row: usize, column: usize) -> impl Iterator<Item = u8> + '_ {
        PointSurroundsIterator::new(self, row, column)
    }

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

    fn low_points(&self) -> impl Iterator<Item = (u8, (usize, usize))> + '_ {
        self.0.iter().enumerate().flat_map(move |(row, v)| {
            v.iter().enumerate().filter_map(move |(column, e)| {
                let mut surrounds = self.get_surrounds(row, column);
                if surrounds.all(|s| s > *e) {
                    Some((*e, (row, column)))
                } else {
                    None
                }
            })
        })
    }

    fn basin_at_with_previous(
        &self,
        row: isize,
        column: isize,
        previous: u8,
        visited: &mut HashSet<(isize, isize)>,
    ) -> usize {
        if visited.contains(&(row, column)) {
            return 0;
        }
        if let Some(current) = self.get(row, column) {
            if current > previous && current < 9 {
                visited.insert((row, column));
                1 + self.basin_at(row, column, visited)
            } else {
                0
            }
        } else {
            0
        }
    }

    fn basin_at(&self, row: isize, column: isize, visited: &mut HashSet<(isize, isize)>) -> usize {
        visited.insert((row, column));
        CHECKS
            .iter()
            .map(|(i, j)| {
                self.basin_at_with_previous(
                    row + *i,
                    column + *j,
                    self.get(row, column).unwrap(),
                    visited,
                )
            })
            .sum()
    }
}

fn solve_part_1(heatmap: &HeatMap) -> usize {
    heatmap.low_points().map(|(e, _)| e as usize + 1usize).sum()
}

fn solve_part_2(heatmap: &HeatMap) -> usize {
    let mut visited = HashSet::new();
    let mut basins: Vec<usize> = heatmap
        .low_points()
        .map(|(e, (row, column))| 1 + heatmap.basin_at(row as isize, column as isize, &mut visited))
        .collect();
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

#[cfg(test)]
mod test {
    use crate::day_09::{solve_part_1, solve_part_2, HeatMap};
    use crate::utils::io;
    use std::io::Read;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn example_part_1() {
        let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

        let heatmap: HeatMap = input.parse().unwrap();
        assert_eq!(solve_part_1(&heatmap), 15);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let heatmap: HeatMap = {
            let mut reader =
                io::open_file_read(&PathBuf::from_str("./inputs/day_09.txt").unwrap())?;
            let mut buff = String::new();
            reader.read_to_string(&mut buff)?;
            buff.parse().unwrap()
        };
        println!("Day 9 part 1 solution: {}", solve_part_1(&heatmap));
        Ok(())
    }

    #[test]
    fn example_part_2() {
        let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

        let heatmap: HeatMap = input.parse().unwrap();
        assert_eq!(solve_part_2(&heatmap), 1134);
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let heatmap: HeatMap = {
            let mut reader =
                io::open_file_read(&PathBuf::from_str("./inputs/day_09.txt").unwrap())?;
            let mut buff = String::new();
            reader.read_to_string(&mut buff)?;
            buff.parse().unwrap()
        };
        println!("Day 9 part 2 solution: {}", solve_part_2(&heatmap));
        Ok(())
    }
}
