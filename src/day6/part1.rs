use std::collections::HashSet;


const FRAME_SIZE: usize = 4;

fn main() {
    let input = std::fs::read_to_string("src/day6/input/input.txt").unwrap();
    let chars: Vec<char> = input.chars().collect();
    let mut control_set = HashSet::new();
    for i in 0..(input.len() - FRAME_SIZE) {
        for j in 0..FRAME_SIZE {
            control_set.insert(chars[i+j]);
        }

        if control_set.len() == FRAME_SIZE {
            println!("index={}", i + FRAME_SIZE);
            println!("chars[index]={}", chars[i + FRAME_SIZE]);
            return;
        }

        control_set.clear();
    }
}
