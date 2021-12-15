use std::collections::HashSet;
use std::str::FromStr;

static CHECKS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

struct RiskMap(Vec<Vec<u8>>);

impl FromStr for RiskMap {
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

impl RiskMap {
    fn lowest_total_risk_from(
        &self,
        size: (isize, isize),
        current_cell: (usize, usize),
        target: (usize, usize),
        mut visited: HashSet<(usize, usize)>,
        mut previous: Vec<(usize, usize)>,
        previous_risk: usize,
    ) -> Option<(Vec<(usize, usize)>, usize)> {
        let (row, column) = current_cell;
        previous.push(current_cell);
        let this_risk = previous_risk + self.0[row][column] as usize;
        if current_cell == target {
            return Some((previous, this_risk));
        }
        visited.insert(current_cell);
        RiskMap::next_steps_from_position(current_cell, size, &visited.clone())
            .into_iter()
            .map(move |next_cell| {
                self.lowest_total_risk_from(
                    size,
                    next_cell,
                    target,
                    visited.clone(),
                    previous.clone(),
                    this_risk,
                )
            })
            .flatten()
            .min_by_key(|(_, risk)| *risk)
    }

    fn next_steps_from_position(
        (row, column): (usize, usize),
        (row_len, column_len): (isize, isize),
        visited: &HashSet<(usize, usize)>,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        CHECKS
            .into_iter()
            .map(move |(r, c)| (row as isize + r, column as isize + c))
            .filter(move |(r, c)| *r >= 0isize && *c >= 0isize && *r < row_len && *c < column_len)
            .filter_map(|(r, c)| {
                let res = (r as usize, c as usize);
                if !visited.contains(&res) {
                    Some(res)
                } else {
                    None
                }
            })
    }
}

fn solve_part_1(riskmap: RiskMap) -> usize {
    let (_steps, total_risk) = riskmap
        .lowest_total_risk_from(
            (riskmap.0.len() as isize, riskmap.0[0].len() as isize),
            (0, 0),
            (riskmap.0.len() - 1, riskmap.0[0].len() - 1),
            HashSet::new(),
            Vec::new(),
            0,
        )
        .unwrap();
    total_risk
}

#[cfg(test)]
mod test {
    use crate::day_15::{solve_part_1, RiskMap};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;
    #[test]
    fn example_part_1() {
        let input = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let riskmap: RiskMap = input.parse().unwrap();
        assert_eq!(solve_part_1(riskmap), 40);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let riskmap: RiskMap =
            io::read_object_from_file(&PathBuf::from_str("./inputs/day_15.txt").unwrap())?;
        let solution = solve_part_1(riskmap);
        println!("Day 15 part 1 solution: {}", solution);
        Ok(())
    }
}
