use std::{collections::HashSet, time::Instant, u16};

fn main() {
    let start = Instant::now();
    // let data = include_str!("input.txt");
    let data = include_str!("test.txt");

    let res = solve(&data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result: {:?}", res);
}

fn solve(data: &str) -> u8{

    let mut all_shapes: Vec<Shape> = Vec::<Shape>::new();

    let mut all_problems: Vec<Problem> = Vec::<Problem>::new();

    parse_input(data, &mut all_shapes, &mut all_problems);

    for f in 0..2{
        let flip = f== 1;
        for r in 0..4{
            let new_matrix = all_shapes[2].clone().get_matrix_with_rotation(r, flip);
            let new_shape = Shape{matrix: new_matrix };
            println!("Rotation: {}, Flip: {}", r, f);
            new_shape.display_shape();
        }
    }

    println!("{:?}", all_problems);
    1
}


#[derive(Debug, Clone)]
struct Shape{
    matrix: HashSet<(u8, u8)>
}

impl Shape {

    fn get_matrix_with_rotation(self, rotation: u8, flip: bool) -> HashSet<(u8, u8)>{
        const MAX_Y:u8 = 2;
        const MAX_X:u8 = 2;
        if rotation == 0 && flip == false{
            return self.matrix;
        }

        let mut new_matrix = HashSet::<(u8, u8)>::with_capacity(self.matrix.len());

        for &(x, y) in &self.matrix {
            let (new_x, new_y) = match rotation {
                1 => (MAX_Y - y, x),
                2 => (MAX_X - x, MAX_Y - y),
                3 => (y, MAX_X - x),
                _ => (x, y),
            };
            new_matrix.insert((new_x, new_y));
        }

        if !flip{
            return new_matrix;
        }

        let mut flipped_matrix = HashSet::<(u8, u8)>::with_capacity(self.matrix.len());

        for (x, y) in new_matrix {
            flipped_matrix.insert((MAX_X - x, y));
        }

        flipped_matrix
    }

    fn display_shape(&self){
        
        for y in 0..3{
            for x in 0..3{
                if self.matrix.contains(&(x, y)){
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

fn parse_input(data: &str, all_shapes: &mut Vec<Shape>, all_problems: &mut Vec<Problem>){

    let mut reading_shape = false;
    let mut nodes = HashSet::<(u8, u8)>::new();

    let mut shape_lines = Vec::<&str>::new();

    for line in data.lines(){
        if line == ""{
            for (y, &line) in shape_lines.iter().enumerate(){
                for (x, c) in line.chars().enumerate(){
                    if c == '#'{
                        nodes.insert((x as u8, y as u8));
                    }
                }
            }

            all_shapes.push(
                Shape { matrix: nodes.clone() }
            );

            nodes.clear();
            shape_lines.clear();
            reading_shape = false;

            continue;
        }
        else if line.contains(" "){

            let mut contained_shapres: Vec<(u16, u16)> = Vec::new();

            let first_split = line.split_once(":").unwrap();
            let sizes:Vec<&str> = first_split.0.split("x").collect();

            let max_x: u16 = sizes
                .get(0)
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(0);

            let max_y: u16 = sizes
                .get(1)
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(0);

            for (i, n) in first_split.1.split_whitespace().enumerate(){
                contained_shapres.push((i as u16, n.parse::<u16>().unwrap()));
            }

            let new_problem = Problem{ matrix_x: max_x, matrix_y: max_y, avaliable_shapes: contained_shapres };
            all_problems.push(new_problem);
        }
        else if !reading_shape {
            reading_shape = true
        }
        else if reading_shape {
            shape_lines.push(line);
        }


    }
}


#[derive(Debug)]
struct Problem {
    matrix_x: u16,
    matrix_y: u16,
    avaliable_shapes: Vec<(u16, u16)>
}