use std::{
    collections::HashSet
    ,
    time::Instant,
};

fn main() {
    let start = Instant::now();
    let data = include_str!("input.txt");
    // let data = include_str!("test.txt");

    let res = solve(&data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result: {:?}", res);
}

fn solve(data: &str) -> u16 {
    let mut all_shapes: Vec<Shape> = Vec::<Shape>::new();
    let mut all_problems: Vec<Problem> = Vec::<Problem>::new();

    let mut res = 0;
    parse_input(data, &mut all_shapes, &mut all_problems);

    let mut mby_can_be_solved = 0;
    let mut cant_be_solved = 0;
    let mut can_be_solved = 0;

    for problem in all_problems {
        let (area, best_way, worst_way) = solve_problem(&problem, &all_shapes);
        if area >= worst_way {
            can_be_solved += 1;
        }
        else if area <= best_way {
            cant_be_solved += 1;
        }else {
            mby_can_be_solved += 1;
        }
    }
    println!("Cant be solved in {}", cant_be_solved);
    println!("Can be solved in {}", can_be_solved);
    println!("Mby be solved in {}", mby_can_be_solved);
    res
}

#[derive(Debug, Clone)]
struct Shape {
    matrix: HashSet<(u16, u16)>,
    space_occupied: u16,
}


fn parse_input(data: &str, all_shapes: &mut Vec<Shape>, all_problems: &mut Vec<Problem>) {
    let mut reading_shape = false;
    let mut nodes = HashSet::<(u16, u16)>::new();

    let mut shape_lines = Vec::<&str>::new();
    let mut space_occupied = 0;
    for line in data.lines() {
        if line == "" {
            for (y, &line) in shape_lines.iter().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c == '#' {
                        nodes.insert((x as u16, y as u16));
                        space_occupied += 1;
                    }
                }
            }

            all_shapes.push(Shape {
                matrix: nodes.clone(),
                space_occupied,
            });

            nodes.clear();
            shape_lines.clear();
            space_occupied = 0;
            reading_shape = false;

            continue;
        } else if line.contains(" ") {
            let mut contained_shapres: Vec<(u16, u16)> = Vec::new();

            let first_split = line.split_once(":").unwrap();
            let sizes: Vec<&str> = first_split.0.split("x").collect();

            let max_x: u16 = sizes
                .get(0)
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(0);

            let max_y: u16 = sizes
                .get(1)
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(0);

            for (i, n) in first_split.1.split_whitespace().enumerate() {
                contained_shapres.push((i as u16, n.parse::<u16>().unwrap()));
            }

            let new_problem = Problem {
                matrix_x: max_x,
                matrix_y: max_y,
                avaliable_shapes: contained_shapres,
            };
            all_problems.push(new_problem);
        } else if !reading_shape {
            reading_shape = true
        } else if reading_shape {
            shape_lines.push(line);
        }
    }
}

#[derive(Debug)]
struct Problem {
    matrix_x: u16,
    matrix_y: u16,
    avaliable_shapes: Vec<(u16, u16)>,
}

fn solve_problem(problem: &Problem, all_shapes: &Vec<Shape>) -> (u16, u16, u16) {

    let area = problem.matrix_x * problem.matrix_y;
    let mut best_occupied_area = 0;
    let mut worst_occupied_area = 0;

    for (present_index, present_amount) in problem.avaliable_shapes.iter() {
        best_occupied_area += *present_amount * all_shapes.get(*present_index as usize).unwrap().space_occupied;
        worst_occupied_area += *present_amount * 9;
    }
    
    (area, best_occupied_area, worst_occupied_area)
}