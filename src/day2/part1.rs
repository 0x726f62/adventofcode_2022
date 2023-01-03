fn main() {
    let input = std::fs::read_to_string("src/day2/input/input.txt").unwrap();

    let mut global_sum: u32 = 0;

    //A rock
    //B paper
    //C Scissors

    //X rock
    //Y paper
    //Z Scissors
    for line in input.lines() {
        let actions_points = match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => 0,
        };

        global_sum += actions_points;
    }

    println!("global_max_sum = {}", global_sum);
}
