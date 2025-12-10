use std::{collections::{HashSet, VecDeque}, time::Instant};

fn main() {
    let start = Instant::now();
    let data = include_str!("input.txt");
    // let data = include_str!("test.txt");

    let res = solve(&data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result: {:?}", res);
}

fn solve(data: &str) -> u64 {
    let mut res = 0;

    for line in data.lines() {
        let parts:Vec<&str> = line.split(" ").collect();

        let target_lights = proccess_target_ligths(parts[0]);
        let buttons:Vec<Vec<u8>> = proccess_buttons(parts[1..parts.len() -1].to_vec());

        // Ignored for now
        // let jolts = parts[parts.len() -1];

        let target_ligth_len = target_lights.len();
        let problem = Problem{
            target_ligths: target_lights,
            curr_lights: vec![0; target_ligth_len],
            button: buttons,
        };

        let solved_count = solve_problem(problem);
        res += solved_count;
    }

    res
}

fn solve_problem(problem: Problem) -> u64 {
    let button_count = problem.button.len();

    let mut problems = VecDeque::<(Problem, u64)>::new();
    let mut past_lights: HashSet<Vec<u8>> = HashSet::new();
    problems.push_back((problem.clone(), 0));

    past_lights.insert(problem.curr_lights.clone());
    loop {
        let (problem, depth) = problems.pop_front().unwrap();
        for i in 0..button_count {
            let mut new_problem = problem.clone();
            new_problem.button_press(i);
            if new_problem.is_solved(){
                return depth +1;
            }
            if !past_lights.contains(&new_problem.curr_lights){
                past_lights.insert(new_problem.curr_lights.clone());
                problems.push_back((new_problem, depth +1));   
            }
        }
    }
}

fn proccess_buttons(values: Vec<&str>) -> Vec<Vec<u8>> {
    let mut buttons = Vec::<Vec<u8>>::new();

    for part in values{
        let mut button = Vec::<u8>::new();
        for c in part.chars(){
            if c.is_numeric(){
                button.push(c.to_digit(10).unwrap() as u8);
            }
        }
        buttons.push(button.clone());
    }
    
    buttons
}

fn proccess_target_ligths(lights: &str) -> Vec<u8> {
    let mut target_lights = Vec::<u8>::new();
    for c in lights.chars() {
        if c != '[' && c != ']' {
            if c == '.' {
                target_lights.push(0);
            } else {
                target_lights.push(1);
            }
        }
    }
    target_lights
}

#[derive(Debug, Clone)]
struct Problem {
    target_ligths: Vec<u8>,
    curr_lights: Vec<u8>,
    button: Vec<Vec<u8>>,
}
impl Problem {
    fn button_press(&mut self, button_index: usize) {
        let rule = self.button.get(button_index as usize).unwrap();
        for &command in rule {
            let new_value = (self.curr_lights[command as usize] + 1) % 2;
            self.curr_lights[command as usize] = new_value;
        }
    }
    fn is_solved(&self) -> bool {
        self.target_ligths == self.curr_lights
    }
}
