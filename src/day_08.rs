use std::collections::{HashMap, HashSet};
use std::lazy::Lazy;
use std::str::FromStr;

type Digit = HashSet<char>;

const DIGITS: Lazy<[Digit; 10]> = Lazy::new(|| {
    [
        "abcefg".chars().collect(),
        "cf".chars().collect(),
        "acdeg".chars().collect(),
        "acdfg".chars().collect(),
        "bcdf".chars().collect(),
        "abdfg".chars().collect(),
        "abdefg".chars().collect(),
        "acf".chars().collect(),
        "abcdefg".chars().collect(),
        "abcdfg".chars().collect(),
    ]
});

const NUMBERS: Lazy<HashMap<String, usize>> =
    Lazy::new(|| DIGITS.iter().map(digit_key).zip(0usize..).collect());

const SIZES: Lazy<[usize; 10]> = Lazy::new(|| DIGITS.clone().map(|set| set.len()));

//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg
fn reverse_count(inputs: &[Digit]) -> HashMap<usize, char> {
    "abcdefg"
        .chars()
        .map(|c| (inputs.iter().filter(|d| d.contains(&c)).count(), c))
        .collect()
}

fn letter_by_count(
    letter: char,
    count: usize,
    by_count: &HashMap<usize, char>,
    mapping: &mut HashMap<char, char>,
    inverse_mapping: &mut HashMap<char, char>,
) {
    let remap = by_count[&count];
    mapping.insert(letter, remap);
    inverse_mapping.insert(remap, letter);
}

fn rule_f(
    by_count: &HashMap<usize, char>,
    mapping: &mut HashMap<char, char>,
    inverse_mapping: &mut HashMap<char, char>,
) {
    letter_by_count('f', 9, by_count, mapping, inverse_mapping);
}

fn rule_e(
    by_count: &HashMap<usize, char>,
    mapping: &mut HashMap<char, char>,
    inverse_mapping: &mut HashMap<char, char>,
) {
    letter_by_count('e', 4, by_count, mapping, inverse_mapping);
}

fn rule_b(
    by_count: &HashMap<usize, char>,
    mapping: &mut HashMap<char, char>,
    inverse_mapping: &mut HashMap<char, char>,
) {
    letter_by_count('b', 6, by_count, mapping, inverse_mapping);
}

fn rule_feb(
    by_count: &HashMap<usize, char>,
    mapping: &mut HashMap<char, char>,
    inverse_mapping: &mut HashMap<char, char>,
) {
    rule_f(by_count, mapping, inverse_mapping);
    rule_e(by_count, mapping, inverse_mapping);
    rule_b(by_count, mapping, inverse_mapping);
}

fn rule_c(
    inputs: &[Digit],
    mapping: &mut HashMap<char, char>,
    inverse_mapping: &mut HashMap<char, char>,
) {
    let one = inputs.iter().find(|d| d.len() == SIZES[1]).unwrap();
    let c = one
        .difference(&[mapping[&'f']].into_iter().collect())
        .copied()
        .next()
        .unwrap();

    mapping.insert('c', c);
    inverse_mapping.insert(c, 'c');
}

fn rule_a(
    inputs: &[Digit],
    mapping: &mut HashMap<char, char>,
    reverse_mapping: &mut HashMap<char, char>,
) {
    let seven = inputs.iter().find(|d| d.len() == SIZES[7]).unwrap();
    let c = mapping[&'c'];
    let f = mapping[&'f'];
    let set: Digit = [c, f].into_iter().collect();
    let a = seven.difference(&set).collect::<Vec<_>>().pop().unwrap();
    mapping.insert('a', *a);
    reverse_mapping.insert(*a, 'a');
}

fn rule_d(
    inputs: &[Digit],
    mapping: &mut HashMap<char, char>,
    reverse_mapping: &mut HashMap<char, char>,
) {
    let checked: Digit = "acf".chars().map(|c| mapping[&c]).collect();
    let subset = inputs
        .iter()
        .filter_map(|d| {
            let diff: Digit = d.difference(&checked).copied().collect();
            if diff.len() == 2 {
                Some(diff)
            } else {
                None
            }
        })
        .reduce(|s1, s2| s1.intersection(&s2).copied().collect())
        .unwrap();
    let d = subset.iter().next().unwrap();
    mapping.insert('d', *d);
    reverse_mapping.insert(*d, 'd');
}

fn rule_g(
    inputs: &[Digit],
    mapping: &mut HashMap<char, char>,
    reverse_mapping: &mut HashMap<char, char>,
) {
    let to_diff: Digit = reverse_mapping.keys().copied().collect();
    let nine = inputs
        .iter()
        .filter_map(|d| {
            let res: Digit = d.difference(&to_diff).copied().collect();
            if res.len() == 1 {
                Some(res)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    let g = nine.iter().next().unwrap();
    mapping.insert('g', *g);
    reverse_mapping.insert(*g, 'g');
}

fn build_mappings(inputs: &[Digit]) -> (HashMap<char, char>, HashMap<char, char>) {
    let mut mapping = HashMap::new();
    let mut reverse_mapping = HashMap::new();
    let by_count = reverse_count(inputs);
    rule_feb(&by_count, &mut mapping, &mut reverse_mapping);
    rule_c(inputs, &mut mapping, &mut reverse_mapping);
    rule_a(inputs, &mut mapping, &mut reverse_mapping);
    rule_d(inputs, &mut mapping, &mut reverse_mapping);
    rule_g(inputs, &mut mapping, &mut reverse_mapping);
    (mapping, reverse_mapping)
}

fn remap_digit(digit: &Digit, mapping: &HashMap<char, char>) -> Digit {
    digit.iter().map(|c| mapping[c]).collect()
}

fn digits_to_num(digits: &[Digit]) -> usize {
    digits
        .iter()
        .map(digit_key)
        .map(|k| NUMBERS[&k])
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

fn digit_key(digit: &Digit) -> String {
    let mut chars: Vec<char> = digit.iter().copied().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

struct Io {
    input: Vec<Digit>,
    output: Vec<Digit>,
}

impl FromStr for Io {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.splitn(2, " | ").collect();
        let input: Vec<Digit> = data[0]
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.chars().collect())
            .collect();

        if input.len() != 10 {
            dbg!("Input len > 10");
            return Err(());
        }

        let output: Vec<Digit> = data[1]
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.chars().collect())
            .collect();

        Ok(Self { input, output })
    }
}

fn solve_part_1(data: &[Io]) -> usize {
    let mut results = {
        let mut map = HashMap::new();
        for i in [1, 4, 7, 8] {
            map.insert(*SIZES.get(i).unwrap(), 0);
        }
        map
    };

    for io in data {
        for digit in &io.output {
            results.entry(digit.len()).and_modify(|e| *e += 1);
        }
    }

    results.values().sum()
}

fn solve_part_2(data: &[Io]) -> usize {
    data.iter()
        .map(|io| {
            let (_, mapping) = build_mappings(&io.input);
            let remapped: Vec<Digit> = io.output.iter().map(|d| remap_digit(d, &mapping)).collect();
            digits_to_num(&remapped)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day_08::{solve_part_1, solve_part_2, Io, DIGITS};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn example_part_1() -> std::io::Result<()> {
        let data: Vec<Io> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_08_example.txt").unwrap())?;

        assert_eq!(solve_part_1(&data), 26);
        Ok(())
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let data: Vec<Io> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_08.txt").unwrap())?;

        println!("Day 8 part 1 solution: {}", solve_part_1(&data));
        Ok(())
    }

    #[test]
    fn research() {
        for c in "abcdefg".chars() {
            println!(
                "{} -> {}",
                c,
                DIGITS.iter().filter(|s| s.contains(&c)).count()
            );
        }
    }

    #[test]
    fn example_part_2() -> std::io::Result<()> {
        let data: Vec<Io> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_08_example.txt").unwrap())?;

        assert_eq!(solve_part_2(&data), 61229);
        Ok(())
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let data: Vec<Io> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_08.txt").unwrap())?;

        println!("Day 8 part 2 solution: {}", solve_part_2(&data));
        Ok(())
    }
}
