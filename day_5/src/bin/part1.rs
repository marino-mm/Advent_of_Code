use std::{collections::BTreeSet, time::Instant};

fn main() {
    let start = Instant::now();

    // let data = "3-5
    //     10-14
    //     16-20
    //     12-18

    //     1
    //     5
    //     8
    //     11
    //     17
    //     32";
    let data = include_str!("input.txt");

    let result = solve(&data);
    let duration = start.elapsed();
    println!("Reslut: {:?}", result);
    println!("Duration: {:?}", duration);
}

fn solve(data: &str) -> i32 {
    //let mut ranges = Vec::<(u64, u64)>::new();
    let mut ranges = BTreeSet::<(u64, u64)>::new();
    let mut reduced_ranges = Vec::<(u64, u64)>::with_capacity(200);
    let mut parse_phase = 0;
    let mut fresh_ingredients = 0;

    for line in data.lines() {
        if line.len() == 0 {
            parse_phase += 1;
            reduced_ranges = reduce_ranges(&ranges);
        }
        else if parse_phase == 0 {
            let range_ends: Vec<&str> = line.trim().split("-").collect();
            
            let start = range_ends[0].parse::<u64>().unwrap();
            let end = range_ends[1].parse::<u64>().unwrap();
            ranges.insert((start, end));
        } else if parse_phase == 1 {
            let ingredient = line.trim().parse::<u64>().unwrap();
            if is_ingredient_fresh(ingredient, &reduced_ranges){
                fresh_ingredients += 1
            }
        }
    }
    fresh_ingredients
}


fn is_ingredient_fresh(ing: u64, ranges: &Vec<(u64,u64)>) -> bool{
    for range in ranges{
        if range.0 <= ing && ing <= range.1{
            return true;
        }
        if range.1 < ing{
            break;
        }
    }
    false
}

fn reduce_ranges(ranges: &BTreeSet<(u64,u64)>) -> Vec<(u64, u64)> {
    
    let mut reduced_ranges = Vec::<(u64, u64)>::with_capacity(ranges.len());
    let mut last_range:Option<(u64, u64)> = None;
    for range in ranges{
        if last_range.is_none(){
            last_range = Some(*range);
        } else{
            let last_range_start = last_range.unwrap().0;
            let last_range_end = last_range.unwrap().1;

            if last_range_end <= range.1 && last_range_end >= range.0{
                last_range = Some((last_range_start, range.1));
            } else if last_range_end > range.1{
                continue;
            } else {
                reduced_ranges.push(last_range.unwrap());
                last_range = Some(*range);
            }
        }
    }
    reduced_ranges.push(last_range.unwrap());
    reduced_ranges
}