fn main() {
    let input = std::fs::read_to_string("src/day8/input/input.txt").unwrap();

    let matrix: Vec<Vec<u32>> = init_vector(&input);
    let matrix_rows = matrix.len();
    let matrix_columns = matrix[0].len();
    let mut global_max_score: u32 = 0;
    let mut current_score_left: u32 = 0;
    let mut current_score_right: u32 = 0;
    let mut current_score_up: u32 = 0;
    let mut current_score_down: u32 = 0;
    let mut current_score: u32 = 0;

    for i in 1..matrix_rows-1 {
        let columns = &matrix[i];
        for j in 1..matrix_columns-1 {
            println!("matrix[{}][{}] = {}", i, j, columns[j]);
            //can compute quadrant and optimize search little bit

            //left
            for p in (0..j).rev() {
                current_score_left += 1;
                if columns[p] >= columns[j] {
                    break;
                }
            }

            //right
            for p in (j+1)..matrix_columns {
                current_score_right += 1;
                if columns[p] >= columns[j] {
                    break;
                }
            }

            //up
            for p in (0..i).rev() {
                current_score_up += 1;
                if matrix[p][j] >= columns[j] {
                    break;
                }
            }

            //down
            for p in (i+1)..matrix_rows {
                current_score_down += 1;
                if matrix[p][j] >= columns[j] {
                    break;
                }
            }
            println!("current_score_left={}", current_score_left);
            println!("current_score_right={}", current_score_right);
            println!("current_score_up={}", current_score_up);
            println!("current_score_down={}", current_score_down);

            current_score = current_score_left * current_score_right * current_score_up * current_score_down;
            println!("current_score={}", current_score);

            if current_score > global_max_score {
                global_max_score = current_score;
                println!("new global_max_score={}", global_max_score);
            }

            current_score_left = 0;
            current_score_right = 0;
            current_score_up = 0;
            current_score_down = 0;
            current_score= 0;
        }
    }

    println!("global_max_score={}", global_max_score);
}

fn init_vector(input: &str) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        let line_vec:Vec<u32>  = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        matrix.push(line_vec);
    }

    matrix
}

