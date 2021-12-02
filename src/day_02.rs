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

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn compute_command(&mut self, command: &Command) {
        match command {
            Command::Forward(x) => {
                self.x += x;
            }
            Command::Down(y) | Command::Up(y) => {
                self.y += y;
            }
        }
    }

    pub fn compute_solution(&self) -> i32 {
        self.x * self.y
    }
}

fn solve_position_part_1(commands: &[Command]) -> i32 {
    let position: Position = commands
        .into_iter()
        .fold(Position::new(), |mut position, command| {
            position.compute_command(command);
            position
        });
    position.compute_solution()
}

#[cfg(test)]
mod test {
    use crate::day_02::*;
    use crate::utils::io;
    use std::path::PathBuf;

    #[test]
    fn example() {
        let input = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(-3),
            Command::Down(8),
            Command::Forward(2),
        ];
        assert_eq!(solve_position_part_1(&input), 150);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let input: Vec<Command> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_02.txt").unwrap())?;
        println!("Day 2 part 1 result: {}", solve_position_part_1(&input));
        Ok(())
    }
}
