use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Polymer(Vec<char>);

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect()))
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    matching: (char, char),
    insertion: char,
}

type Rules = HashMap<(char, char), char>;

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (char1, char2, insertion) = sscanf::scanf!(s, "{}{} -> {}", char, char, char).unwrap();
        Ok(Self {
            matching: (char1, char2),
            insertion,
        })
    }
}

impl Polymer {
    fn apply_rules(&self, rules: &HashMap<(char, char), char>) -> Polymer {
        Self(
            self.0
                .windows(2)
                .flat_map(|pair| -> Box<dyn Iterator<Item = char>> {
                    let (a, b): (char, char) = {
                        let mut it = pair.iter();
                        (*it.next().unwrap(), *it.next().unwrap())
                    };
                    rules
                        .get(&(a, b))
                        .map(|matching| -> Box<dyn Iterator<Item = char>> {
                            Box::new([a, *matching].into_iter())
                        })
                        .unwrap_or_else(|| Box::new([a].into_iter()))
                })
                .chain(std::iter::once(*self.0.last().unwrap()))
                .collect(),
        )
    }

    fn min_max(&self) -> (usize, usize) {
        let map: HashMap<char, usize> = self.0.iter().fold(HashMap::new(), |mut map, e| {
            *map.entry(*e).or_default() += 1;
            map
        });
        (
            *map.iter().min_by_key(|(_, v)| **v).unwrap().1,
            *map.iter().max_by_key(|(_, v)| **v).unwrap().1,
        )
    }
}

fn parse_input(mut reader: impl BufRead) -> (Polymer, Rules) {
    let mut buff = String::new();
    reader.read_to_string(&mut buff).unwrap();
    let mut parts = buff.splitn(2, "\r\n\r\n");
    (
        parts.next().unwrap().parse().unwrap(),
        parts
            .next()
            .unwrap()
            .lines()
            .map(|l| l.parse::<Rule>().unwrap())
            .map(|r| (r.matching, r.insertion))
            .collect(),
    )
}

fn solve(polymer: Polymer, rules: &Rules, iters: usize) -> Polymer {
    (0..iters).fold(polymer, |polymer, _| polymer.apply_rules(rules))
}

fn solve_part_1(polymer: Polymer, rules: &Rules) -> usize {
    let result = solve(polymer, rules, 10);
    let (min, max) = result.min_max();
    max - min
}

fn solve_2(polymer: Polymer, rules: &Rules, iterations: usize) -> usize {
    let mut results: HashMap<(char, char), usize> = HashMap::new();

    for pair in polymer.0.windows(2) {
        let mut iter = pair.iter();
        let (&a, &b) = (iter.next().unwrap(), iter.next().unwrap());
        *results.entry((a, b)).or_default() += 1;
    }

    for _ in 0..iterations {
        results = {
            let mut inner_results: HashMap<(char, char), usize> = HashMap::new();
            for (&(a, b), &count) in &results {
                if let Some(&insertion) = rules.get(&(a, b)) {
                    *inner_results.entry((a, insertion)).or_default() += count;
                    *inner_results.entry((insertion, b)).or_default() += count;
                } else {
                    *inner_results.entry((a, b)).or_default() += count;
                }
            }
            inner_results
        };
    }

    let aggregated_results: HashMap<char, usize> =
        results
            .iter()
            .fold(HashMap::new(), |mut map, (&(a, _b), &value)| {
                *map.entry(a).or_default() += value;
                map
            });

    let (min, max) = (
        *aggregated_results
            .iter()
            .min_by_key(|(_, v)| **v)
            .unwrap()
            .1,
        *aggregated_results
            .iter()
            .max_by_key(|(_, v)| **v)
            .unwrap()
            .1,
    );
    max - min + 1
}

#[cfg(test)]
mod test {
    use crate::day_14::{parse_input, solve_2, solve_part_1};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn parse_example() -> std::io::Result<()> {
        let reader =
            io::open_file_read(&PathBuf::from_str("./inputs/day_14_example.txt").unwrap())?;
        let (polymer, rules) = parse_input(reader);
        assert_eq!(polymer.0.len(), 4);
        assert_eq!(rules.len(), 16);
        Ok(())
    }

    #[test]
    fn example_part_1() -> std::io::Result<()> {
        let reader =
            io::open_file_read(&PathBuf::from_str("./inputs/day_14_example.txt").unwrap())?;
        let (polymer, rules) = parse_input(reader);
        let res = solve_part_1(polymer, &rules);
        assert_eq!(res, 1588);
        Ok(())
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let reader = io::open_file_read(&PathBuf::from_str("./inputs/day_14.txt").unwrap())?;
        let (polymer, rules) = parse_input(reader);
        println!("Day 14 part 1 solution: {}", solve_part_1(polymer, &rules));
        Ok(())
    }

    #[test]
    fn example_part_2() -> std::io::Result<()> {
        let reader =
            io::open_file_read(&PathBuf::from_str("./inputs/day_14_example.txt").unwrap())?;
        let (polymer, rules) = parse_input(reader);
        assert_eq!(
            solve_part_1(polymer.clone(), &rules),
            solve_2(polymer, &rules, 10)
        );
        Ok(())
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let reader = io::open_file_read(&PathBuf::from_str("./inputs/day_14.txt").unwrap())?;
        let (polymer, rules) = parse_input(reader);
        println!("Day 14 part 2 solution: {}", solve_2(polymer, &rules, 40));
        Ok(())
    }
}
