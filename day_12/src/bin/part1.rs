use std::{
    collections::{HashSet, VecDeque},
    process::exit,
    time::Instant,
};

fn main() {
    let start = Instant::now();
    // let data = include_str!("input.txt");
    let data = include_str!("test.txt");

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

    for (index, problem) in all_problems.iter().enumerate() {
        println!("Problem {}.", index + 1);
        println!("{:?}", problem);
        if solve_problem(problem, &all_shapes) {
            res += 1;
        }
    }
    res
}

#[derive(Debug, Clone)]
struct Shape {
    matrix: HashSet<(u16, u16)>,
}

impl Shape {
    fn get_matrix_with_rotation(self, rotation: u16, flip: bool) -> HashSet<(u16, u16)> {
        const MAX_Y: u16 = 2;
        const MAX_X: u16 = 2;
        if rotation == 0 && flip == false {
            return self.matrix;
        }

        let mut new_matrix = HashSet::<(u16, u16)>::with_capacity(self.matrix.len());

        for &(x, y) in &self.matrix {
            let (new_x, new_y) = match rotation {
                1 => (MAX_Y - y, x),
                2 => (MAX_X - x, MAX_Y - y),
                3 => (y, MAX_X - x),
                _ => (x, y),
            };
            new_matrix.insert((new_x, new_y));
        }

        if !flip {
            return new_matrix;
        }

        let mut flipped_matrix = HashSet::<(u16, u16)>::with_capacity(self.matrix.len());

        for (x, y) in new_matrix {
            flipped_matrix.insert((MAX_X - x, y));
        }

        flipped_matrix
    }

    fn display_shape(&self) {
        for y in 0..3 {
            for x in 0..3 {
                if self.matrix.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}

fn parse_input(data: &str, all_shapes: &mut Vec<Shape>, all_problems: &mut Vec<Problem>) {
    let mut reading_shape = false;
    let mut nodes = HashSet::<(u16, u16)>::new();

    let mut shape_lines = Vec::<&str>::new();

    for line in data.lines() {
        if line == "" {
            for (y, &line) in shape_lines.iter().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c == '#' {
                        nodes.insert((x as u16, y as u16));
                    }
                }
            }

            all_shapes.push(Shape {
                matrix: nodes.clone(),
            });

            nodes.clear();
            shape_lines.clear();
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

fn solve_problem(problem: &Problem, all_shapes: &Vec<Shape>) -> bool {
    let max_x = problem.matrix_x;
    let max_y = problem.matrix_y;
    let mut plane: Vec<Vec<u16>> = vec![vec![0; max_x.into()]; max_y.into()];

    let mut shapes = VecDeque::<Shape>::new();

    for (index, amount) in problem.avaliable_shapes.clone() {
        if amount > 0 {
            for _ in 0..amount {
                shapes.push_front(all_shapes.get(index as usize).unwrap().clone());
            }
        }
    }
    let mut new_plane = plane.clone();

    for shape in shapes {
        new_plane = fit_new_shape(&new_plane, &shape, all_viable_starting_points(&new_plane));
        print_plane(&new_plane);
    }

    true
}

fn fit_new_shape(plane: &Vec<Vec<u16>>,new_shape: &Shape,starting_positions: Vec<(u16, u16)>,) -> Vec<Vec<u16>> {
    let max_y = plane.len();
    let max_x = plane[0].len();

    let mut viable_new_plane = HashSet::new();

    for (start_x, start_y) in starting_positions {
        for flip in 0..2 {
            for rotation in 0..4 {
                let new_point = new_shape
                    .clone()
                    .get_matrix_with_rotation(rotation, flip == 1);
                let mut new_plane: Vec<Vec<u16>> = plane.clone();
                let mut shape_fits = true;

                'shape_poop: for (x, y) in new_point {
                    let new_y = (y + start_y) as usize;
                    let new_x = (x + start_x) as usize;

                    if new_y < max_y && new_x < max_x && new_plane[new_y][new_x] == 0 {
                        new_plane[new_y][new_x] += 1;
                    } else {
                        shape_fits = false;
                        break 'shape_poop;
                    }
                }

                if shape_fits {
                    viable_new_plane.insert(new_plane);
                }
            }
        }
    }

    if viable_new_plane.len() == 0{
        return plane.to_vec();
    } else {

        let mut all_planes: Vec<(usize, Vec<Vec<u16>>)> = Vec::new();

        for plane in viable_new_plane.iter(){
            for (r_idx, row) in plane.iter().enumerate().rev() {
                for (c_idx, &value) in row.iter().enumerate().rev() {
                    if value == 1 {
                        all_planes.push((r_idx + c_idx, plane.clone()));
                    }
                }
            }
        }
        all_planes.sort_by_key(|item| item.0);


        return all_planes.first().unwrap().1.clone();
    }
}

fn print_plane(plane: &Vec<Vec<u16>>) {
    for row in plane {
        for &column in row {
            if column > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn all_viable_starting_points(plane: &Vec<Vec<u16>>) -> Vec<(u16, u16)> {
    let max_y = plane.len();
    let max_x = plane[0].len();

    let mut starting_poss = HashSet::<(u16, u16)>::new();

    starting_poss.insert((0, 0));

    for y in 0..max_y {
        for x in 0..max_x {
            if plane[y][x] == 1 {
                for var_x in -1isize..2 {
                    for var_y in -1..2 {
                        let new_x = x as isize + var_x;
                        let new_y = y as isize + var_y;
                        if new_x >= 0
                            && new_x < max_x as isize
                            && new_y >= 0
                            && new_y < max_y as isize
                        {
                            let new_x = new_x as usize;
                            let new_y = new_y as usize;

                            if plane[new_y][new_x] == 0 {
                                starting_poss.insert((new_x as u16, new_y as u16));
                            }
                        }
                    }
                }
            }
        }
    }

    starting_poss.into_iter().collect()
}
