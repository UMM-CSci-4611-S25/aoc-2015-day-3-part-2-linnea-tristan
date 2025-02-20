use std::{collections::HashSet, ops::Add, str::FromStr};

fn main() {
    let input_file_name = "input.txt";
    let contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    println!("{}", contents.len());
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32
}

impl Add<Direction> for Pos {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => Self::new(self.x, self.y + 1),
            Direction::South => Self::new(self.x, self.y - 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }
}

impl Pos {
    fn new(x: i32, y:i32) -> Self {
        Self {x, y}
    }
}

struct  VisitedHouses {
    visited_houses: HashSet<Pos>,
    current_position: Pos,
    robo_current_position: Pos,
    real: bool
}

impl VisitedHouses {
    #[must_use]
    fn new() -> Self {
        let mut visited_houses: HashSet<Pos> = HashSet::new();
        let initial_position: Pos = Pos::new(0, 0);
        visited_houses.insert(initial_position);

        VisitedHouses{
            visited_houses,
            current_position: initial_position,
            robo_current_position: initial_position,
            real: true
        }
    }

    #[must_use]
    fn current_pos(&self) -> Pos {
        self.current_position.clone()
    }

    fn num_visited_houses(&self) -> usize {
        self.visited_houses.len()
    }

    fn perform_move(&mut self, direction:Direction) {
        let new_position;
        if self.real {
            new_position = self.current_position + direction;
            self.current_position = new_position;
        } else {
            new_position = self.robo_current_position + direction;
            self.robo_current_position = new_position;
        }

        self.real = !self.real;

        self.visited_houses.insert(new_position);
        

    }
    fn perform_moves(&mut self, moves: Moves) {
        for m in moves.moves {
            self.perform_move(m);
        }
    }
}

struct Moves {
    moves: Vec<Direction>,
}

impl TryFrom<char> for Direction {
    type Error = IllegalChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Self::North),
            'v' => Ok(Self::South),
            '<' => Ok(Self::West),
            '>' => Ok(Self::East),
            _ => Err(IllegalChar(c))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalChar(char);

impl FromStr for Moves {
    type Err = IllegalChar;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .chars()
            .map(Direction::try_from)
            .collect::<Result<Vec<Direction>, IllegalChar>>()?;
        Ok(Self { moves })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visited_houses_new() {
        let mut visited_houses = VisitedHouses::new();
        assert_eq!(visited_houses.num_visited_houses(), 1);
        // What do you want to do about the current position?
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_direction_try_from() {
        assert_eq!('^'.try_into(), Ok(Direction::North));
        assert_eq!('v'.try_into(), Ok(Direction::South));
        assert_eq!('<'.try_into(), Ok(Direction::West));
        assert_eq!('>'.try_into(), Ok(Direction::East));
        assert_eq!(Direction::try_from('x'), Err(IllegalChar('x')));
    }

    #[test]
    fn test_move_north_south() {
        let mut visited_houses = VisitedHouses::new();
        visited_houses.perform_move(Direction::North);
        visited_houses.perform_move(Direction::South);
        assert_eq!(visited_houses.num_visited_houses(), 3);
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 1));
    }

    #[test]
    fn test_square_moves() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str("^>v<").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 3);
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_up_down_moves() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str("^v^v^v^v^v").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 11);
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 5));
    }

    #[test]
    fn test_aoc_input() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str(include_str!("../../input.txt")).unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 2639);
    }
}
