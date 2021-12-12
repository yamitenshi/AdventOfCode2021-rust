pub struct Position {
    pub depth: i32,
    pub horizontal_position: i32,
}

impl Position {
    fn apply(&self, instruction: Instruction) -> Position {
        return match instruction.direction {
            Direction::Forward => Position { depth: self.depth, horizontal_position: self.horizontal_position + instruction.amount as i32 },
            Direction::Down => Position { depth: self.depth + instruction.amount as i32, horizontal_position: self.horizontal_position },
            Direction::Up => Position { depth: self.depth - instruction.amount as i32, horizontal_position: self.horizontal_position },
            Direction::None => Position { depth: self.depth, horizontal_position: self.horizontal_position },
        }
    }
}

enum Direction {
    Forward,
    Down,
    Up,
    None,
}

impl From<&str> for Direction {
    fn from(direction: &str) -> Self {
        match direction {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => Direction::None,
        }
    }
}

struct Instruction {
    direction: Direction,
    amount: u32
}

impl From<&str> for Instruction {
    fn from(instruction: &str) -> Self {
        let parts: Vec<&str> = instruction.split(' ').collect();
        if parts.len() != 2 {
            return Instruction{
                direction: Direction::None,
                amount: 0
            };
        }

        return Instruction {
            direction: Direction::from(parts[0]),
            amount: parts[1].parse().unwrap()
        }
    }
}

pub fn navigate<'a, I>(instructions: I) -> Position
where
    I: IntoIterator<Item = &'a String>
{
    return instructions.into_iter()
        .map(|instruction| Instruction::from(instruction as &str))
        .fold(
            Position {
                depth: 0,
                horizontal_position: 0
            },
            |current_position, instruction| current_position.apply(instruction)
        )
    ;
}