use std::time::Instant;

use good_lp::{Expression, Solution, SolverModel, variable, variables};
use good_lp::solvers::coin_cbc::coin_cbc;


fn main() {
    let start = Instant::now();
    //let data = include_str!("input.txt");
    // let data = include_str!("test.txt");
    let data = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";

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

    
}