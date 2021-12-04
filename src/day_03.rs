use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Entry<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> FromStr for Entry<SIZE> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s
            .chars()
            .map(|c| c.eq(&'1').then_some(1u8).unwrap_or(0))
            .collect::<Vec<u8>>();
        assert_eq!(bits.len(), SIZE);
        Ok(Self::try_from(bits).unwrap())
    }
}

impl<const SIZE: usize> Entry<SIZE> {
    fn to_number(&self) -> u32 {
        self.0
            .iter()
            .zip((0..SIZE as u32).rev())
            .map(|(bit, exp)| *bit as u32 * 2u32.pow(exp))
            .sum()
    }

    fn reverse(&self) -> Self {
        Self(
            <[u8; SIZE]>::try_from(
                self.0
                    .iter()
                    .copied()
                    .map(|b| match b {
                        1 => 0,
                        _ => 1,
                    })
                    .collect::<Vec<u8>>(),
            )
            .unwrap(),
        )
    }

    fn has_n_at(&self, n: u8, i: usize) -> bool {
        self.0[i] == n
    }
}

impl<const SIZE: usize> TryFrom<Vec<u8>> for Entry<SIZE> {
    type Error = <[u8; SIZE] as TryFrom<Vec<u8>>>::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(<[u8; SIZE]>::try_from(value)?))
    }
}

fn find_common_in_column<const SIZE: usize>(
    entries: &[Entry<SIZE>],
    column: usize,
    default: u8,
    common: bool,
) -> u8 {
    let (zeros, ones) =
        entries
            .iter()
            .map(|entry| entry.0[column])
            .fold((0usize, 0usize), |(zeros, ones), e| match e {
                0 => (zeros + 1, ones),
                1 => (zeros, ones + 1),
                _ => unreachable!(),
            });
    match zeros.cmp(&ones) {
        Ordering::Less => common.then_some(1).unwrap_or(0),
        Ordering::Equal => default,
        Ordering::Greater => common.then_some(0).unwrap_or(1),
    }
}

fn find_common_uncommon<const SIZE: usize>(entries: &[Entry<SIZE>]) -> (Entry<SIZE>, Entry<SIZE>) {
    let common: Vec<u8> = (0..SIZE)
        .map(|i| find_common_in_column(entries, i, 1, true))
        .collect();
    let common: Entry<SIZE> = Entry::try_from(common).unwrap();
    let uncommon = common.reverse();
    (common, uncommon)
}

fn solve_part1<const SIZE: usize>(entries: &[Entry<SIZE>]) -> u32 {
    let (common, uncommon) = find_common_uncommon(entries);
    common.to_number() * uncommon.to_number()
}

fn filter_trendy<const SIZE: usize>(mut entries: Vec<Entry<SIZE>>, common: bool) -> u32 {
    for i in 0..SIZE {
        if entries.len() == 1 {
            break;
        }
        let column_common =
            find_common_in_column(&entries, i, common.then_some(1).unwrap_or(0), common);
        entries = entries
            .drain(..)
            .filter(|entry| entry.has_n_at(column_common, i))
            .collect();
    }
    entries.pop().unwrap().to_number()
}

fn solve_part2<const SIZE: usize>(entries: Vec<Entry<SIZE>>) -> u32 {
    let common = filter_trendy(entries.clone(), true);
    let uncommon = filter_trendy(entries, false);
    common * uncommon
}

#[cfg(test)]
mod test {
    use crate::day_03::{solve_part1, solve_part2, Entry};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn example_part1() {
        let entries = vec![
            Entry([0, 0, 1, 0, 0]),
            Entry([1, 1, 1, 1, 0]),
            Entry([1, 0, 1, 1, 0]),
            Entry([1, 0, 1, 1, 1]),
            Entry([1, 0, 1, 0, 1]),
            Entry([0, 1, 1, 1, 1]),
            Entry([0, 0, 1, 1, 1]),
            Entry([1, 1, 1, 0, 0]),
            Entry([1, 0, 0, 0, 0]),
            Entry([1, 1, 0, 0, 1]),
            Entry([0, 0, 0, 1, 0]),
            Entry([0, 1, 0, 1, 0]),
        ];

        assert_eq!(solve_part1(&entries), 198);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let entries: Vec<Entry<12>> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_03.txt").unwrap())?;
        println!("Day 3 part 1 result: {}", solve_part1(&entries));
        Ok(())
    }

    #[test]
    fn example_part2() {
        let entries = vec![
            Entry([0, 0, 1, 0, 0]),
            Entry([1, 1, 1, 1, 0]),
            Entry([1, 0, 1, 1, 0]),
            Entry([1, 0, 1, 1, 1]),
            Entry([1, 0, 1, 0, 1]),
            Entry([0, 1, 1, 1, 1]),
            Entry([0, 0, 1, 1, 1]),
            Entry([1, 1, 1, 0, 0]),
            Entry([1, 0, 0, 0, 0]),
            Entry([1, 1, 0, 0, 1]),
            Entry([0, 0, 0, 1, 0]),
            Entry([0, 1, 0, 1, 0]),
        ];

        assert_eq!(solve_part2(entries), 230);
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let entries: Vec<Entry<12>> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_03.txt").unwrap())?;
        println!("Day 3 part 2 result: {}", solve_part2(entries));
        Ok(())
    }
}
