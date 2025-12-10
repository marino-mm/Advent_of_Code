use std::{
    collections::{HashSet, VecDeque},
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

fn solve(data: &str) -> u64 {
    let mut res = 0;

    for line in data.lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        // let target_lights = proccess_target_ligths(parts[0]);
        let buttons: Vec<Vec<u8>> = proccess_buttons(parts[1..parts.len() - 1].to_vec());

        let target_jolts = proccess_jolts(parts.last().unwrap());

        let target_jolt_len = target_jolts.len();
        let problem = Problem {
            target_jolts: target_jolts,
            curr_jolts: vec![0; target_jolt_len],
            button: buttons,
        };

        let solved_count = solve_problem(problem);
        println!("{}", solved_count);
        res += solved_count;
    }

    res
}


fn solve_problem(problem: Problem) -> u64 {
    let button_count = problem.button.len();

    let mut problems = VecDeque::<(Problem, u64)>::new();
    let mut past_jolts: HashSet<Vec<u8>> = HashSet::new();
    problems.push_front((problem.clone(), 0));

    past_jolts.insert(problem.curr_jolts.clone());
    loop {
        let (problem, depth) = match problems.pop_front() {
            Some(state) => state,
            None => { println!("Stack is empty!!!!");return 0},
        };
        for i in 0..button_count {
            let mut new_problem = problem.clone();
            new_problem.button_press(i);
            if new_problem.is_solved() {
                return depth + 1;
            }
            if new_problem.is_valid(){
                if !past_jolts.contains(&new_problem.curr_jolts){
                    past_jolts.insert(new_problem.curr_jolts.clone());
                    // println!("{:?}", problem);
                    problems.push_front((new_problem, depth + 1));
                }
            }
        }
    }
}


fn proccess_jolts(values: &str) -> Vec<u8> {

    let mut values = values.replace("{", "");
    values = values.replace("}", "");

    values
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect()
}

fn proccess_buttons(values: Vec<&str>) -> Vec<Vec<u8>> {
    let mut buttons = Vec::<Vec<u8>>::new();

    for part in values {
        let mut button = Vec::<u8>::new();
        for c in part.chars() {
            if c.is_numeric() {
                button.push(c.to_digit(10).unwrap() as u8);
            }
        }
        buttons.push(button.clone());
    }

    buttons
}

#[allow(dead_code)]
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
    target_jolts: Vec<u8>,
    curr_jolts: Vec<u8>,
    button: Vec<Vec<u8>>,
}
impl Problem {
    fn button_press(&mut self, button_index: usize) {
        let rule = self.button.get(button_index as usize).unwrap();
        for &command in rule {
            let new_value = self.curr_jolts[command as usize] + 1;
            self.curr_jolts[command as usize] = new_value;
        }
    }
    fn is_solved(&self) -> bool {
        self.target_jolts == self.curr_jolts
    }
    fn is_valid(&self) -> bool {
        for i in 0..self.curr_jolts.len(){
            let curr_jolt = self.curr_jolts.get(i).unwrap();
            let target_jolt = self.target_jolts.get(i).unwrap();
            if curr_jolt > target_jolt{
                return false;
            }
        }
        true
    }
}
