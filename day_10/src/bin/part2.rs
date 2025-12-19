use std::{collections::HashSet, time::Instant};

fn main() {
    let start = Instant::now();
    // let data = include_str!("input.txt");
    // let data = include_str!("test.txt");
    // let data = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    // let data = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    // let data = "[##.#..##.] (0,2,3,5,7,8) (1,2,8) (2,6,8) (0,2,4,6,7,8) (0,1,8) (0,4,5,8) (1,3) (0,1,3,4,8) (0,2,5,8) (5) (0,1,2,3,5,7) {77,51,54,27,31,69,6,17,94}";
    let data = "[#.#.] (0,2) (1,3) {4,1,4,1}";

    let res = solve(&data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result: {:?}", res);
}

fn solve(data: &str) -> i32 {
    let mut res = 0;

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
    let mut matrix = Vec::<Vec<f64>>::new();

    let mut row_set: HashSet<Vec<i64>> = HashSet::new();
    for (c_i, constant) in problem.target_jolts.iter().enumerate() {
        let mut row: Vec<i64> = Vec::new();
        for button in &problem.button {
            if button.contains(&(c_i as u64)) {
                row.push(1);
            } else {
                row.push(0);
            }
        }
        row.push(*constant as i64);
        if !row_set.contains(&row) {
            matrix.push(row.iter().map(|&x| x as f64).collect());
            row_set.insert(row);
        }
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut lead = 0;pretty_print_matrix(&matrix);

    for r in 0..rows {
        if lead >= cols {
            break;
        }
        let mut i = r;
        while matrix[i][lead].abs() < 1e-9 {
            i += 1;
            if i == rows {
                i = r;
                lead += 1;
                if lead == cols {
                    return matrix;
                }
            }
        }
        matrix.swap(i, r);
        let div = matrix[r][lead];
        for j in 0..cols {
            matrix[r][j] /= div;
        }
        for i in 0..rows {
            if i != r {
                let matrixult = matrix[i][lead];
                for j in 0..cols {
                    matrix[i][j] -= matrixult * matrix[r][j];
                }
            }
        }
        lead += 1;
    }
    pretty_print_matrix(&matrix);
}

fn pretty_print_matrix(matrix: &Vec<Vec<f64>>) {
    for row in matrix {
        for value in row {
            print!("{} ", value);
        }
        print!("\n");
    }
    print!("\n");
}
