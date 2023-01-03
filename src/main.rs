fn main() {


    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut global_max_sum: u32 = 0;
    let mut tmp_sum: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            if tmp_sum > global_max_sum {
                global_max_sum = tmp_sum;
            }
            tmp_sum = 0;
            continue;
        }
        tmp_sum += line.parse::<u32>().unwrap();
    }

    println!("global_max_sum = {}", global_max_sum);
}
