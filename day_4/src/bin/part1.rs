use std::{collections::BTreeSet, time::Instant};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PaperRoll {
    x: i64,
    y: i64,
}

fn solve(data: &str) -> u64 {
    let mut accessed_rolls = 0;

    let mut roll_set = BTreeSet::<PaperRoll>::new();

    for (line_i, line) in data.lines().enumerate() {
        for (char_i, c) in line.chars().enumerate() {
            if c == '@' {
                roll_set.insert(PaperRoll { x: char_i as i64, y: line_i as i64});
            }
        }
    }
    
    for paper_roll in &roll_set{
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
            accessed_rolls += 1
        }
    }
    accessed_rolls
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
