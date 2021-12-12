pub struct Position {
    pub depth: i32,
    pub horizontal_position: i32,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            depth: self.depth,
            horizontal_position: self.horizontal_position
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.depth = source.depth;
        self.horizontal_position = source.horizontal_position;
    }
}

impl Position {
    fn apply(&self, instruction: Instruction) -> Position {
        return match instruction.direction {
            Direction::Forward => Position { depth: self.depth, horizontal_position: self.horizontal_position + instruction.amount },
            Direction::Down => Position { depth: self.depth + instruction.amount, horizontal_position: self.horizontal_position },
            Direction::Up => Position { depth: self.depth - instruction.amount, horizontal_position: self.horizontal_position },
            Direction::None => self.clone(),
        }
    }
}

pub struct PositionAndAim {
    pub position: Position,
    pub aim: i32
}

impl Clone for PositionAndAim {
    fn clone(&self) -> Self {
        PositionAndAim {
            position: self.position.clone(),
            aim: self.aim
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.position.clone_from(&source.position);
        self.aim = source.aim;
    }
}

impl PositionAndAim {
    fn apply(&self, instruction: Instruction) -> PositionAndAim {
        return match instruction.direction {
            Direction::Forward => PositionAndAim {
                position: Position {
                    horizontal_position: self.position.horizontal_position + instruction.amount,
                    depth: self.position.depth + (self.aim * instruction.amount)
                },
                aim: self.aim,
            },
            Direction::Up => PositionAndAim {
                position: self.position.clone(),
                aim: self.aim - instruction.amount,
            },
            Direction::Down => PositionAndAim {
                position: self.position.clone(),
                aim: self.aim + instruction.amount,
            },
            Direction::None => self.clone()
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
    amount: i32
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

pub fn better_navigate<'a, I>(instructions: I) -> PositionAndAim
where
    I: IntoIterator<Item = &'a String>
{
    return instructions.into_iter()
        .map(|instruction| Instruction::from(instruction as &str))
        .fold(
            PositionAndAim {
                position: Position {
                    depth: 0,
                    horizontal_position: 0,
                },
                aim: 0
            },
            |current, instruction| current.apply(instruction)
        )
    ;
}