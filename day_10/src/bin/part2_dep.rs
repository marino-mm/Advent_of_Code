use std::time::Instant;

use nalgebra::{DMatrix, DVector};

fn main() {
    let start = Instant::now();
    let data = include_str!("input.txt");
    // let data = include_str!("test.txt");
    // let data = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";

    let res = solve(&data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result: {:?}", res);
}

fn solve(data: &str) -> i32 {
    let mut res:i32 = 0;

    for (i, line) in data.lines().enumerate() {
        let parts: Vec<&str> = line.split(" ").collect();

        let buttons: Vec<Vec<u64>> = process_buttons(parts[1..parts.len() - 1].to_vec());
        let target_jolts = process_jolts(parts.last().unwrap());
        let problem = Problem {
            target_jolts: target_jolts,
            button: buttons,
        };

        res += solve_problem(&problem);
        println!("Problem: {}.", i);
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
    buttons
}

fn solve_problem(problem: &Problem) -> i32 {
    let row_count = problem.target_jolts.len();
    let column_count = problem.button.len();
    let mut init_vec = Vec::<f64>::new();

    for (c_i, _constant) in problem.target_jolts.iter().enumerate() {
        for button in &problem.button {
            if button.contains(&(c_i as u64)) {
                init_vec.push(1f64);
            } else {
                init_vec.push(0f64);
            }
        }
    }

    let a = DMatrix::from_row_slice(row_count, column_count, &init_vec);
    let b_data: Vec<f64> = problem.target_jolts
        .iter()
        .map(|&x| x as f64)
        .collect();
    let b = DVector::from_column_slice(&b_data);

    let mut best_sum = i32::MAX;
    let mut residuals = b.clone();

    find_min_sum_optimized(&a, &mut residuals, 0, 0, &mut best_sum);

    if best_sum == i32::MAX { 0 } else { best_sum }
}

fn find_min_sum_optimized(
    a: &DMatrix<f64>,
    residuals: &mut DVector<f64>,
    index: usize,
    current_running_sum: i32,
    best_sum: &mut i32,
) {
    if current_running_sum >= *best_sum {
        return;
    }
    if index == a.ncols() {

        if residuals.iter().all(|&r| r.abs() < 1e-9) {
            *best_sum = current_running_sum;
        }
        return;
    }

    let mut max_presses = i32::MAX;
    let mut contributes_to_any = false;
    
    for r in 0..a.nrows() {
        if a[(r, index)] > 0.0 {
            contributes_to_any = true;
            let possible = (residuals[r] / a[(r, index)]).floor() as i32;
            if possible < max_presses {
                max_presses = possible;
            }
        }
    }

    if !contributes_to_any { max_presses = 0; }

    for val in (0..=max_presses).rev() {
        for r in 0..a.nrows() {
            residuals[r] -= a[(r, index)] * (val as f64);
        }

        find_min_sum_optimized(
            a,
            residuals,
            index + 1,
            current_running_sum + val,
            best_sum,
        );
        for r in 0..a.nrows() {
            residuals[r] += a[(r, index)] * (val as f64);
        }
    }
}