use std::{mem::{swap, take}, time::Instant};

fn main() {
    let start = Instant::now();
    // let data = include_str!("input.txt");
    // let data = include_str!("test.txt");
    let data = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";

    let res = solve(&data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result: {:?}", res);
}

fn solve(data: &str) -> u64 {
    let res = 0;

    for line in data.lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        // let target_lights = proccess_target_ligths(parts[0]);
        let buttons: Vec<Vec<u64>> = process_buttons(parts[1..parts.len() - 1].to_vec());
        let target_jolts = process_jolts(parts.last().unwrap());
        let problem = Problem {
            target_jolts: target_jolts,
            button: buttons,
        };

        solve_problem(&problem)
    }

    res
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    target_jolts: Vec<u64>,
    button: Vec<Vec<u64>>,
}

fn process_jolts(values: &str) -> Vec<u64> {
    let mut values = values.replace("{", "");
    values = values.replace("}", "");

    values
        .split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn process_buttons(values: Vec<&str>) -> Vec<Vec<u64>> {
    let mut buttons = Vec::<Vec<u64>>::new();

    for part in values {
        let mut button = Vec::<u64>::new();
        for c in part.chars() {
            if c.is_numeric() {
                button.push(c.to_digit(10).unwrap() as u64);
            }
        }
        buttons.push(button.clone());
    }

    // buttons.sort_by(|a, b| b.len().cmp(&a.len()));

    buttons
}

fn solve_problem(problem: &Problem) {
    let mut matrix = Vec::<Vec<i64>>::new();

    for (c_i, constant) in problem.target_jolts.iter().enumerate() {
        let mut row = Vec::new();
        for button in &problem.button {
            if button.contains(&(c_i as u64)) {
                row.push(1i64);
            } else {
                row.push(0i64);
            }
        }
        row.push(*constant as i64);
        matrix.push(row);
    }

    let row_count = matrix.len();
    let col_count = matrix[0].len();

    pretty_print_matrix(&matrix);
    print!("\n");
    // Order rows
    for col_i in 0..col_count -1{
        for row_i in col_i..row_count{
            if matrix[row_i][col_i] == 1 && row_i != col_i{
                println!("Swap row {} and row {}", row_i, col_i);
                matrix.swap(row_i, col_i);
            }
        }
    }

    pretty_print_matrix(&matrix);
    // Eliminate

    for row_i in (1 .. row_count).rev(){
        for col_i in (0 .. col_count -1).rev(){
            let pivot = matrix[row_i][col_i];
            let pivot_above = matrix[row_i - 1][col_i];
            //Subtract rows
            if pivot == 1 && pivot_above == 1 {
                for elm_col in (0 .. col_count).rev(){
                    matrix[row_i - 1][elm_col] -=  matrix[row_i][elm_col]
                }
            }
        }
    }
    println!("After elimination");
    pretty_print_matrix(&matrix);

    let var_MIN = 0;
    let mut var_MAX = 0;

    let mut free_var = Vec::<u64>::new();

    for row in &matrix{
        var_MAX = var_MAX.max(*row.last().unwrap())
    }

    let mut pivot_numb = 0;
    for col_i in 0..col_count -1{
        pivot_numb = 0;
        for row_i in 0 .. row_count{
            if matrix[row_i][col_i] == 1{
                pivot_numb += 1
            }
        }
        if pivot_numb > 1{
            free_var.push(col_i as u64);
        }
    }


    println!("Min: {}, Max: {}", var_MIN, var_MAX);
    println!("Free var: {:?}", free_var);

}


fn pretty_print_matrix(matrix: &Vec<Vec<i64>>) {
    for row in matrix {
        for value in row {
            print!("{} ", value);
        }
        print!("\n");
    }
    print!("\n");
}
