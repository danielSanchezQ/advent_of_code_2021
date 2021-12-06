use std::str::FromStr;

#[derive(Debug)]
struct LanternFish<const BASE: usize> {
    counter: usize,
}

impl<const BASE: usize> LanternFish<BASE> {
    fn new(base_counter: usize) -> Self {
        Self {
            counter: base_counter,
        }
    }

    fn dec(&mut self) -> bool {
        if self.counter == 0 {
            self.counter = BASE;
            true
        } else {
            self.counter -= 1;
            false
        }
    }
}

impl<const BASE: usize> FromStr for LanternFish<BASE> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            counter: s.parse().map_err(|_| ())?,
        })
    }
}

fn solve_part_1<const BASE: usize, const EXTRA: usize>(
    mut fish: Vec<LanternFish<BASE>>,
    iter: usize,
) -> usize {
    for _ in 0..iter {
        let new_fish = fish
            .iter_mut()
            .map(|fish| fish.dec())
            .filter(|e| matches!(e, true))
            .count();
        fish.extend((0..new_fish).map(|_| LanternFish::<BASE>::new(BASE + EXTRA)));
    }
    fish.len()
}

fn solve_part_2(fishes: Vec<usize>, days: usize) -> usize {
    let mut fish_days = vec![0usize; 9];
    for fish in &fishes {
        fish_days[*fish] += 1;
    }
    for _ in 0usize..days {
        let fish_0 = fish_days.remove(0);
        fish_days.push(fish_0);
        fish_days[6] += fish_days[8];
    }
    fish_days.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::day_06::{solve_part_1, solve_part_2, LanternFish};
    use crate::utils::io;
    use std::io::Read;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn example_part_1() {
        const BASE: usize = 6;
        const EXTRA: usize = 2;
        let fish = vec![
            LanternFish::new(3),
            LanternFish::new(4),
            LanternFish::new(3),
            LanternFish::new(1),
            LanternFish::new(2),
        ];
        assert_eq!(solve_part_1::<BASE, EXTRA>(fish, 80), 5934);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        const BASE: usize = 6;
        const EXTRA: usize = 2;
        let fish: Vec<LanternFish<BASE>> = {
            let mut reader =
                io::open_file_read(&PathBuf::from_str("./inputs/day_06.txt").unwrap())?;
            let mut buff = String::new();
            reader.read_to_string(&mut buff)?;
            buff.trim().split(',').map(|s| s.parse().unwrap()).collect()
        };
        let result = solve_part_1::<BASE, EXTRA>(fish, 80);
        println!("Day 6 part 1 result: {}", result);
        Ok(())
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let fish: Vec<usize> = {
            let mut reader =
                io::open_file_read(&PathBuf::from_str("./inputs/day_06.txt").unwrap())?;
            let mut buff = String::new();
            reader.read_to_string(&mut buff)?;
            buff.trim().split(',').map(|s| s.parse().unwrap()).collect()
        };
        let result = solve_part_2(fish, 256);
        println!("Day 6 part 2 result: {}", result);
        Ok(())
    }
}
