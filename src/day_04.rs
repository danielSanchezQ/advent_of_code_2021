use std::collections::HashMap;
use std::io::BufRead;

type LotteryNumbers = Vec<usize>;

#[derive(Clone)]
struct Board {
    board: Vec<Vec<usize>>,
    checked: Vec<Vec<bool>>,
    cache: HashMap<usize, (usize, usize)>,
}

impl Board {
    fn from_reader(mut reader: impl BufRead, board_size: usize) -> Self {
        let mut board = Vec::new();
        for _ in 0..board_size {
            let mut buff = String::new();
            reader.read_line(&mut buff).unwrap();
            board.push(
                buff.trim()
                    .split(' ')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse().unwrap())
                    .collect(),
            );
        }
        let mut ret = Board {
            board,
            checked: vec![vec![false; board_size]; board_size],
            cache: Default::default(),
        };
        ret.cached();
        ret
    }

    fn cached(&mut self) {
        for row in 0..self.board.len() {
            for column in 0..self.board[row].len() {
                self.cache.insert(self.board[row][column], (row, column));
            }
        }
    }

    fn process_lottery_number(&mut self, number: usize) -> bool {
        if let Some(&(row, column)) = self.cache.get(&number) {
            self.checked[row][column] = true;
            self.check_bingo_by_row_and_column(row, column)
        } else {
            false
        }
    }

    fn check_bingo_by_row_and_column(&self, row: usize, column: usize) -> bool {
        self.checked[row].iter().all(|e| *e) || self.checked.iter().map(|e| e[column]).all(|e| e)
    }

    fn process_uncheck(&self) -> usize {
        self.checked
            .iter()
            .enumerate()
            .map(|(row, v)| {
                v.iter()
                    .enumerate()
                    .map(|(column, checked)| {
                        (!checked).then_some(self.board[row][column]).unwrap_or(0)
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn read_challenge_input(
    mut reader: impl BufRead,
    board_size: usize,
) -> (LotteryNumbers, Vec<Board>) {
    let lottery_numbers = {
        let mut buff = String::new();
        reader.read_line(&mut buff).unwrap();
        buff.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect()
    };

    let boards = {
        let mut boards = Vec::new();
        loop {
            let mut buff = String::new();
            if reader.read_line(&mut buff).unwrap() == 0 {
                break;
            }
            boards.push(Board::from_reader(&mut reader, board_size));
        }
        boards
    };

    (lottery_numbers, boards)
}

fn solve_part_1(lottery_numbers: LotteryNumbers, mut boards: Vec<Board>) -> usize {
    for n in lottery_numbers {
        for board in boards.iter_mut() {
            if board.process_lottery_number(n) {
                return board.process_uncheck() * n;
            }
        }
    }
    0
}

fn solve_part_2(lottery_numbers: LotteryNumbers, mut boards: Vec<Board>) -> usize {
    let mut winner: Option<(Board, usize)> = None;
    for n in lottery_numbers {
        boards.retain_mut(|board| {
            if board.process_lottery_number(n) {
                winner.replace((board.clone(), n));
                false
            } else {
                true
            }
        });
    }
    if let Some((board, n)) = winner {
        board.process_uncheck() * n
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::day_04::{read_challenge_input, solve_part_1, solve_part_2};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn read_input() -> std::io::Result<()> {
        let mut reader = io::open_file_read(&PathBuf::from_str("./inputs/day_04.txt").unwrap())?;
        let (lottery_numbers, boards) = read_challenge_input(&mut reader, 5);
        assert!(!lottery_numbers.is_empty());
        assert!(!boards.is_empty());
        Ok(())
    }

    #[test]
    fn example_part_1() -> std::io::Result<()> {
        let mut reader =
            io::open_file_read(&PathBuf::from_str("./inputs/day_04_example.txt").unwrap())?;
        let (lottery_numbers, boards) = read_challenge_input(&mut reader, 5);
        let result = solve_part_1(lottery_numbers, boards);
        assert_eq!(result, 4512);
        Ok(())
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let mut reader = io::open_file_read(&PathBuf::from_str("./inputs/day_04.txt").unwrap())?;
        let (lottery_numbers, boards) = read_challenge_input(&mut reader, 5);
        let result = solve_part_1(lottery_numbers, boards);
        println!("Day 4 part 1 result: {}", result);
        Ok(())
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let mut reader = io::open_file_read(&PathBuf::from_str("./inputs/day_04.txt").unwrap())?;
        let (lottery_numbers, boards) = read_challenge_input(&mut reader, 5);
        let result = solve_part_2(lottery_numbers, boards);
        println!("Day 4 part 2 result: {}", result);
        Ok(())
    }
}
