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

    let mut set = HashSet::new();

    for line in input.lines() {
        let (first, second) = line.split_at(line.len()/2);

        // println!("first.len: {}", first.len());
        // println!("second.len: {}", second.len());
        if first.len() != second.len() {
            println!("xxxxxxxxxxxxxsecond.len: {}", second.len());
        }

        println!("first: {}", first);
        println!("second: {}", second);
        for char in first.chars() {
            set.insert(char);
        }

        for char in second.chars() {
            if set.contains(&char) {
                let inc = convert_char_to_ascii_int_value(char);
                global_sum += inc;
                println!("inc: {}", inc);
                println!("char: {}", char);
                println!("global_sum: {}", global_sum);
                break;
            }

        }

        set.clear();
    }

    fn convert_char_to_ascii_int_value(char: char) -> u32 {
        let mut ascii_int_val: u32 = char as u32;

        match ascii_int_val > 96 {
            true => ascii_int_val - 96,
            false => ascii_int_val - 38,
        }
    }

    println!("global_sum: {}", global_sum);
}
