#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("src/day3/input/input.txt").unwrap();
    let mut global_sum: u32 = 0;

    let ch1 = 'a';
    let ch11 = 'z';

    let ch2 = 'A';
    let ch22 = 'Z';

    println!("ASCII value {}: {}",ch1, ch1 as u32);
    println!("ASCII value {}: {}",ch11, ch11 as u32);
    println!("ASCII value {}: {}",ch2, ch2 as u32);
    println!("ASCII value {}: {}",ch22, ch22 as u32);

    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();

    for [line1, line2, line3] in input.lines().array_chunks() {
        println!("line1: {}", line1);
        println!("line2: {}", line2);
        println!("line3: {}", line3);

        for char in line1.chars() {
            set1.insert(char);
        }

        for char in line2.chars() {
            if set1.contains(&char) {
                set2.insert(char);
            }
        }

        for char in line3.chars() {
            if set2.contains(&char) {
                let inc = convert_char_to_ascii_int_value(char);
                global_sum += inc;
                println!("inc: {}", inc);
                println!("char: {}", char);
                println!("global_sum: {}", global_sum);
                break;
            }

        }

        set1.clear();
        set2.clear();
    }

    fn convert_char_to_ascii_int_value(char: char) -> u32 {
        let ascii_int_val: u32 = char as u32;

        match ascii_int_val > 96 {
            true => ascii_int_val - 96,
            false => ascii_int_val - 38,
        }
    }

    println!("global_sum: {}", global_sum);
}
