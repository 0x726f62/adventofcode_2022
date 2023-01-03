fn main() {
    let input = std::fs::read_to_string("src/day1/input/input.txt").unwrap();

    let mut top1: u32 = 0;
    let mut top2: u32 = 0;
    let mut top3: u32 = 0;
    let mut tmp_sum: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            if tmp_sum > top1 {
                top3 = top2;
                top2 = top1;
                top1 = tmp_sum;
            } else if tmp_sum > top2 {
                top3 = top2;
                top2 = tmp_sum;
            } else if tmp_sum > top3 {
                top3 = tmp_sum;
            }
            tmp_sum = 0;
            continue;
        }
        tmp_sum += line.parse::<u32>().unwrap();
    }

    println!("top1 = {}", top1);
    println!("top2 = {}", top2);
    println!("top3 = {}", top3);
    println!("sum 3 tops = {}", top1 + top2 + top3);
}
