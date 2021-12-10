use std::collections::{HashMap, VecDeque};
use std::convert::Infallible;
use std::lazy::Lazy;
use std::str::FromStr;

const CORRUPTED_SCORES: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect()
});

const INCOMPLETE_SCORES: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    [(')', 1), (']', 2), ('}', 3), ('>', 4)]
        .into_iter()
        .collect()
});

const OPPOSITES: Lazy<HashMap<char, char>> = Lazy::new(|| {
    [
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]
    .into_iter()
    .collect()
});

// Navigation subsystem syntax line
struct Nssl(String);

impl FromStr for Nssl {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
enum NsslState {
    Complete,
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn incomplete_score(delimiters: &[char]) -> usize {
    delimiters
        .iter()
        .fold(0, |accum, entry| accum * 5 + INCOMPLETE_SCORES[entry])
}

impl NsslState {
    fn as_score(&self) -> usize {
        match self {
            NsslState::Complete => 0,
            NsslState::Incomplete(delimiters) => incomplete_score(delimiters),
            NsslState::Corrupted(c) => CORRUPTED_SCORES[c],
        }
    }
}

impl Nssl {
    fn state(&self) -> NsslState {
        let mut stack = VecDeque::new();
        for c in self.0.chars() {
            match c {
                c @ ('(' | '[' | '{' | '<') => stack.push_back(c),
                c @ (')' | ']' | '}' | '>') => {
                    if let Some(initial) = stack.pop_back() {
                        if OPPOSITES[&c] != initial {
                            return NsslState::Corrupted(c);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        if stack.is_empty() {
            NsslState::Complete
        } else {
            NsslState::Incomplete(stack.into_iter().rev().map(|c| OPPOSITES[&c]).collect())
        }
    }
}

fn solve_part_1(lines: &[Nssl]) -> usize {
    lines
        .iter()
        .map(|nssl| nssl.state())
        .filter(|state| matches!(state, NsslState::Corrupted(_)))
        .map(|state| state.as_score())
        .sum()
}

fn solve_part_2(lines: &[Nssl]) -> usize {
    let mut res: Vec<_> = lines
        .iter()
        .map(|nssl| nssl.state())
        .filter(|s| matches!(s, NsslState::Incomplete(_)))
        .map(|s| s.as_score())
        .collect();
    res.sort_unstable();
    res[res.len() / 2]
}

#[cfg(test)]
mod test {
    use crate::day_10::{solve_part_1, solve_part_2, Nssl};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn example_part_1() {
        let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let lines: Vec<Nssl> = input.lines().map(|l| Nssl(l.to_string())).collect();
        assert_eq!(solve_part_1(&lines), 26397);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let lines: Vec<Nssl> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_10.txt").unwrap())?;
        println!("Day 10 part 1 solution: {}", solve_part_1(&lines));
        Ok(())
    }

    #[test]
    fn example_part_2() {
        let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let lines: Vec<Nssl> = input.lines().map(|l| Nssl(l.to_string())).collect();
        assert_eq!(solve_part_2(&lines), 288957);
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let lines: Vec<Nssl> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_10.txt").unwrap())?;
        println!("Day 10 part 2 solution: {}", solve_part_2(&lines));
        Ok(())
    }
}
