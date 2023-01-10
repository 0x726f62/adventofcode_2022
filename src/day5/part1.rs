const CRATES_LINES_INPUT: usize = 8;

struct Crate {
    content: char
}

impl Crate {
    fn new(content: char) -> Crate {
        Crate {content}
    }
}


fn main() {
    let input = std::fs::read_to_string("src/day5/input/input.txt").unwrap();

    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();
    let mut stack3 = Vec::new();
    let mut stack4 = Vec::new();
    let mut stack5 = Vec::new();
    let mut stack6 = Vec::new();
    let mut stack7 = Vec::new();
    let mut stack8 = Vec::new();
    let mut stack9 = Vec::new();

    let mut stacks = Vec::new();

    for line in input.lines().take(CRATES_LINES_INPUT) {

        // [T]             [P]     [J]
        // [F]     [S]     [T]     [R]     [B]
        // [V]     [M] [H] [S]     [F]     [R]
        // [Z]     [P] [Q] [B]     [S] [W] [P]
        // [C]     [Q] [R] [D] [Z] [N] [H] [Q]
        // [W] [B] [T] [F] [L] [T] [M] [F] [T]
        // [S] [R] [Z] [V] [G] [R] [Q] [N] [Z]
        // [Q] [Q] [B] [D] [J] [W] [H] [R] [J]
        // 1   2   3   4   5   6   7   8   9


        let chars: Vec<char> = line.chars().collect();

        if !chars[1].is_ascii_whitespace() {
            stack1.push(Crate::new(chars[1]));
        }

        if !chars[5].is_ascii_whitespace() {
            stack2.push(Crate::new(chars[5]));
        }

        if !chars[9].is_ascii_whitespace() {
            stack3.push(Crate::new(chars[9]));
        }

        if !chars[13].is_ascii_whitespace() {
            stack4.push(Crate::new(chars[13]));
        }

        if !chars[17].is_ascii_whitespace() {
            stack5.push(Crate::new(chars[17]));
        }

        if !chars[21].is_ascii_whitespace() {
            stack6.push(Crate::new(chars[21]));
        }

        if !chars[25].is_ascii_whitespace() {
            stack7.push(Crate::new(chars[25]));
        }

        if !chars[29].is_ascii_whitespace() {
            stack8.push(Crate::new(chars[29]));
        }

        if !chars[33].is_ascii_whitespace() {
            stack9.push(Crate::new(chars[33]));
        }

        stack1.reverse();
        stack2.reverse();
        stack3.reverse();
        stack4.reverse();
        stack5.reverse();
        stack6.reverse();
        stack7.reverse();
        stack8.reverse();
        stack9.reverse();

        stacks.push(&mut stack1);
        stacks.push(&mut stack2);
        stacks.push(&mut stack3);
        stacks.push(&mut stack4);
        stacks.push(&mut stack5);
        stacks.push(&mut stack6);
        stacks.push(&mut stack7);
        stacks.push(&mut stack8);
        stacks.push(&mut stack9);
    }

    // move 3 from 8 to 2
    for line in input.lines().skip(CRATES_LINES_INPUT + 2) {
        let splits: Vec<&str> = line.split(' ').collect();
        let count = splits[1].parse::<usize>().unwrap();
        let from = splits[3].parse::<usize>().unwrap();
        let to = splits[5].parse::<usize>().unwrap();

        for _ in 0..count {
            stacks[to-1].push(stacks[from-1].pop().unwrap());
        }
    }
}
