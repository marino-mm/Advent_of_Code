use std::{collections::HashSet, hash::Hash, time::Instant};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PaperRoll {
    x: i64,
    y: i64,
}

fn solve(data: &str) -> u64 {
    let mut accessed_rolls = 0;

    let mut roll_set = HashSet::<PaperRoll>::new();
    generate_set(data, &mut roll_set);
    
    let mut current_roll_set = roll_set;
    let mut new_roll_set ;
    loop {
        new_roll_set = run_sim(&mut accessed_rolls, &current_roll_set);
        if new_roll_set.len() == current_roll_set.len(){
            break;
        }
        current_roll_set = new_roll_set;
    }

    accessed_rolls
}

fn run_sim(accessed_rolls: &mut u64, roll_set: &HashSet<PaperRoll>) -> HashSet<PaperRoll> {
    let mut new_roll_set = HashSet::<PaperRoll>::new();
    for paper_roll in roll_set{
        let mut neighbourgh_numb = 0;
        for var_x in -1..=1{
            for var_y in -1..=1{
                if !(var_x == 0 && var_y == 0){
                    if roll_set.contains(&PaperRoll { x: paper_roll.x + var_x, y: paper_roll.y + var_y }){
                        neighbourgh_numb += 1
                    }
                }
            }
        }
        if neighbourgh_numb < 4 {
            *accessed_rolls += 1
        }
        else {
            new_roll_set.insert(paper_roll.clone());
        }
    }
    new_roll_set
}

fn generate_set(data: &str, roll_set: &mut HashSet<PaperRoll>) {
    for (line_i, line) in data.lines().enumerate() {
        for (char_i, c) in line.chars().enumerate() {
            if c == '@' {
                roll_set.insert(PaperRoll { x: char_i as i64, y: line_i as i64});
            }
        }
    }
}

fn main() {
    /*
    let data = "..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";
     */
    let data = include_str!("input.txt");

    let start = Instant::now();
    let result = solve(&data);
    let duration = start.elapsed();
    println!("Result: {:?}", result);
    println!("Time elapsed: {:?}", duration);
}
