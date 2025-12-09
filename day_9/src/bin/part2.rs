use std::{time::Instant, u64};

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

    let mut tile_count_max = u64::MIN;
    let mut points = Vec::<(u64, u64)>::new();

    for line in data.lines(){
        let parts:Vec<u64> = 
            line
            .split(',')
            .map(|p| p.parse::<u64>().unwrap())
            .collect();
        let curr_point = (*parts.get(0).unwrap(), *parts.get(1).unwrap());
        points.push(curr_point);
    }

    let points_count = points.len();

    

    tile_count_max
}