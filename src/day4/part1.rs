fn main() {
    let input = std::fs::read_to_string("src/day4/input/input.txt").unwrap();

    let mut global_max_sum: u32 = 0;

    for line in input.lines() {
        let intervals: Vec<&str> = line.split(',').collect();
        let first_vals: Vec<&str> = intervals[0].split('-').collect();
        let second_vals: Vec<&str> = intervals[1].split('-').collect();

        let first_vals_begin_num = first_vals[0].parse::<u32>().unwrap();
        let first_vals_end_num = first_vals[1].parse::<u32>().unwrap();
        let second_vals_begin_num = second_vals[0].parse::<u32>().unwrap();
        let second_vals_end_num = second_vals[1].parse::<u32>().unwrap();


        println!("--------------------------------------------------",);
        println!("first_vals_begin_num = {}", first_vals_begin_num);
        println!("first_vals_end_num = {}", first_vals_end_num);
        println!("second_vals_begin_num = {}", second_vals_begin_num);
        println!("second_vals_end_num = {}", second_vals_end_num);
        //123-965,1-96
        //2-965,1-96
        if first_vals_begin_num < second_vals_begin_num {
            if first_vals_end_num >= second_vals_end_num {
                //1st case - bigger & enclosing 2nd
                global_max_sum+=1;
                println!("1st global_max_sum = {}", global_max_sum);
            }
        } else if first_vals_begin_num == second_vals_begin_num {
            //2nd case - one of them always encloses the other
            global_max_sum+=1;
            println!("2nd global_max_sum = {}", global_max_sum);
        } else {
            if second_vals_end_num >= first_vals_end_num {
                //3nd case - bigger & enclosing 1st
                global_max_sum+=1;
                println!("2nd global_max_sum = {}", global_max_sum);
            }
        }

    }

    println!("global_max_sum = {}", global_max_sum);
}
