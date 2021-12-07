use std::collections::HashMap;

fn solve(positions: &[usize], mut calc_fuel: impl FnMut(usize) -> usize) -> (usize, usize) {
    let max = *positions.iter().max().unwrap();
    let mut map: HashMap<usize, usize> = (0..max).zip(std::iter::repeat(0usize)).collect();
    let len = positions.len();
    for &a in positions {
        for b in 0..max {
            let diff = calc_fuel(a.abs_diff(b));
            map.entry(b).and_modify(|e| *e += diff);
        }
    }
    map.iter()
        .min_by_key(|(pos, fuel)| **fuel)
        .map(|(&pos, &fuel)| (pos, fuel))
        .unwrap()
}

fn solve_part_1(positions: &[usize]) -> (usize, usize) {
    solve(positions, |i| i)
}

fn solve_part_2(positions: &[usize]) -> (usize, usize) {
    // it would be better to cache the fuel calculations, but laziness
    solve(positions, |i| (1..=i).sum())
}

#[cfg(test)]
mod test {
    use crate::day_07::{solve_part_1, solve_part_2};
    use crate::utils::io;
    use std::io::Read;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn example_part_1() {
        let positions = vec![16usize, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(solve_part_1(&positions), (2, 37))
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let positions: Vec<usize> = {
            let mut reader =
                io::open_file_read(&PathBuf::from_str("./inputs/day_07.txt").unwrap())?;
            let mut buff = String::new();
            reader.read_to_string(&mut buff)?;
            buff.split(',').map(|s| s.parse().unwrap()).collect()
        };

        let (pos, fuel) = solve_part_1(&positions);
        println!("Day 7 part 1 solution: pos {}, fuel {}", pos, fuel);
        Ok(())
    }

    #[test]
    fn example_part_2() {
        let positions = vec![16usize, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(solve_part_2(&positions), (5, 168))
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let positions: Vec<usize> = {
            let mut reader =
                io::open_file_read(&PathBuf::from_str("./inputs/day_07.txt").unwrap())?;
            let mut buff = String::new();
            reader.read_to_string(&mut buff)?;
            buff.split(',').map(|s| s.parse().unwrap()).collect()
        };

        let (pos, fuel) = solve_part_2(&positions);
        println!("Day 7 part 2 solution: pos {}, fuel {}", pos, fuel);
        Ok(())
    }
}
