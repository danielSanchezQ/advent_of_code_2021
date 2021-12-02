use std::str::FromStr;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, value): (String, i32) = sscanf::scanf!(s, "{} {}", String, i32).ok_or(())?;
        Ok(match command.to_lowercase().as_str() {
            "forward" => Self::Forward(value),
            "down" => Self::Down(value),
            "up" => Self::Up(-value),
            _ => return Err(()),
        })
    }
}

trait Day2Solver {
    fn compute_command(&mut self, command: &Command);
    fn compute_solution(&self) -> i32;
}

struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }
}

impl Day2Solver for Position {
    fn compute_command(&mut self, command: &Command) {
        match command {
            Command::Forward(x) => {
                self.horizontal += x;
            }
            Command::Down(y) | Command::Up(y) => {
                self.depth += y;
            }
        }
    }

    fn compute_solution(&self) -> i32 {
        self.horizontal * self.depth
    }
}

struct Aimed {
    position: Position,
    aim: i32,
}

impl Aimed {
    pub fn new() -> Self {
        Self {
            position: Position::new(),
            aim: 0,
        }
    }
}

impl Day2Solver for Aimed {
    fn compute_command(&mut self, command: &Command) {
        match command {
            Command::Forward(x) => {
                self.position.horizontal += x;
                self.position.depth += self.aim * x;
            }
            Command::Down(aim) | Command::Up(aim) => {
                self.aim += aim;
            }
        }
    }

    fn compute_solution(&self) -> i32 {
        self.position.compute_solution()
    }
}

fn solve_position_with_solver<Solver: Day2Solver>(solver: Solver, commands: &[Command]) -> i32 {
    let position: Box<dyn Day2Solver> =
        Box::new(commands.into_iter().fold(solver, |mut position, command| {
            position.compute_command(command);
            position
        }));
    position.compute_solution()
}

#[cfg(test)]
mod test {
    use crate::day_02::*;
    use crate::utils::io;
    use std::path::PathBuf;

    #[test]
    fn example_part_1() {
        let input = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(-3),
            Command::Down(8),
            Command::Forward(2),
        ];
        assert_eq!(solve_position_with_solver(Position::new(), &input), 150);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let input: Vec<Command> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_02.txt").unwrap())?;
        println!(
            "Day 2 part 1 result: {}",
            solve_position_with_solver(Position::new(), &input)
        );
        Ok(())
    }

    #[test]
    fn example_part_2() {
        let input = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(-3),
            Command::Down(8),
            Command::Forward(2),
        ];
        assert_eq!(solve_position_with_solver(Aimed::new(), &input), 900);
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let input: Vec<Command> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_02.txt").unwrap())?;
        println!(
            "Day 2 part 2 result: {}",
            solve_position_with_solver(Aimed::new(), &input)
        );
        Ok(())
    }
}
