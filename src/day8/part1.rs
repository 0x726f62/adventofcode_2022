fn main() {
    let input = std::fs::read_to_string("src/day8/input/input.txt").unwrap();

    let matrix: Vec<Vec<u32>> = init_vector(&input);
    let matrix_rows = matrix.len();
    let matrix_columns = matrix[0].len();
    let mut visible_trees = 0;

    println!("matrix_rows = {}", matrix_rows);
    println!("matrix_columns = {}", matrix_columns);

    let visible_outer_trees = matrix_rows + matrix_rows + (matrix_columns - 1) + (matrix_columns - 1);

    for i in 1..matrix_rows-1 {
        let columns = &matrix[i];
        for j in 1..matrix_columns-1 {
            let mut visible: bool = true;
            println!("matrix[{}][{}] = {}", i, j, columns[j]);
            //can compute quadrant and optimize search little bit

            for p in j..0 {
                if columns[p] >= columns[j] {
                    visible = false;
                    break;
                }
            }
            //check flag
            if visible == true {
                visible_trees += 1;
                continue;
            }

            for p in j..matrix_columns {
                if columns[p] >= columns[j] {
                    visible = false;
                    break;
                }
            }
            //check flag
            if visible == true {
                visible_trees += 1;
                continue;
            }
            for p in i..0 {
                if matrix[p][j] >= columns[j] {
                    visible = false;
                    break;
                }
            }
            //check flag
            if visible == true {
                visible_trees += 1;
                continue;
            }
            for p in i..matrix_rows {
                if matrix[p][j] >= columns[j] {
                    visible = false;
                    break;
                }
            }
            //check flag
            if visible == true {
                visible_trees += 1;
                continue;
            }
    }

    }

    println!("visible_trees={}", visible_trees);
    println!("visible_outer_trees={}", visible_outer_trees);
    println!("all_visible_trees={}", visible_trees + visible_outer_trees);
}

fn init_vector(input: &str) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        let line_vec:Vec<u32>  = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        matrix.push(line_vec);
    }

    matrix
}

