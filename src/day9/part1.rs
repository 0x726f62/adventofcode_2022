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

    fn do_move(&mut self, direction: &Direction, distance: &i32) {
        match direction {
            Direction::UP => self.y += distance,
            Direction::DOWN => self.y -= distance,
            Direction::LEFT => self.x -= distance,
            Direction::RIGHT => self.x += distance,
        }
    }

    fn do_jump(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn toString(&self) -> String {
        return format!("{},{}", self.x.to_string(), self.y.to_string());
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

//I know but it works for our cases :-)
fn detect_disc(head: &Coord, tail: &Coord) -> bool {
    //top left
    if (tail.x == head.x - 1) && (tail.y == head.y + 1) {
        return false;
    }
    //top
    if (tail.x == head.x ) && (tail.y == head.y + 1) {
        return false;
    }
    //top right
    if (tail.x == head.x + 1) && (tail.y == head.y + 1) {
        return false;
    }
    //mid left
    if (tail.x == head.x - 1) && (tail.y == head.y) {
        return false;
    }
    //mid
    if head.eq(&tail) {
        return false;
    }
    //mid right
    if (tail.x == head.x + 1) && (tail.y == head.y) {
        return false;
    }
    //bot left
    if (tail.x == head.x - 1) && (tail.y == head.y - 1) {
        return false;
    }
    //bot
    if (tail.x == head.x ) && (tail.y == head.y - 1) {
        return false;
    }
    //bot right
    if (tail.x == head.x + 1) && (tail.y == head.y - 1) {
        return false;
    }

    return true;
}

fn main() {
    let input = std::fs::read_to_string("src/day9/input/input.txt").unwrap();

    let mut control_set = HashSet::new();

    let mut head = Coord::new(0, 0);
    let mut tail =Coord::new(0, 0);

    println!("head = {}", head.toString());
    println!("tail = {}", tail.toString());

    for line in input.lines() {
        let intervals: Vec<&str> = line.split(' ').collect();

        println!("line = {}", line);

        let direction = parse(intervals[0]).unwrap();
        let distance = intervals[1].parse::<i32>().unwrap();

        for _ in 0..distance {
            head.do_move(&direction, &1);

            if detect_disc(&head, &tail) {
                println!("disc");
                match direction {
                    Direction::UP => tail.do_jump(head.x, head.y - 1),
                    Direction::DOWN => tail.do_jump(head.x, head.y + 1),
                    Direction::LEFT => tail.do_jump(head.x + 1, head.y),
                    Direction::RIGHT => tail.do_jump(head.x - 1, head.y),
                }
            }

            println!("head = {}", head.toString());
            println!("tail = {}", tail.toString());

            control_set.insert(tail.toString());
        }
    }

    println!("control_set.len() = {}", control_set.len());
}