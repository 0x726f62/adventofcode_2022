use std::collections::HashSet;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}


#[derive(Debug, PartialEq, Eq)]
struct ParseCoordError;

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn do_move(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::UP => self.y += distance,
            Direction::DOWN => self.y -= distance,
            Direction::LEFT => self.x += distance,
            Direction::RIGHT => self.x -= distance,
        }
    }
}

fn parse(str: &str) -> Result<Direction, ParseCoordError> {
    let direction = match str {
        "U" => Direction::UP,
        "D" => Direction::DOWN,
        "L" => Direction::LEFT,
        "R" => Direction::RIGHT,
        _ => return Err(ParseCoordError),
    };

    Ok(direction)
}

fn main() {
    let input = std::fs::read_to_string("src/day9/input/input.txt").unwrap();

    let mut control_set = HashSet::new();

    for line in input.lines() {
        let intervals: Vec<&str> = line.split(' ').collect();

        let direction = parse(intervals[0]).unwrap();
        let distance = intervals[1].parse::<usize>().unwrap();
    }
}