use std::{collections::BinaryHeap, time::Instant};

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

    let mut tile_count_heap = BinaryHeap::<u64>::new();
    let mut points = Vec::<(u64, u64)>::new();

    for line in data.lines(){
        let parts:Vec<u64> = 
            line
            .split(',')
            .map(|p| p.parse::<u64>().unwrap())
            .collect();
        let curr_point = (*parts.get(0).unwrap(), *parts.get(1).unwrap());
        points.push(curr_point);

        for &point in &points{
            if point != curr_point{
                let dist = (point.0.abs_diff(curr_point.0) + 1) * (point.1.abs_diff(curr_point.1) + 1);
                tile_count_heap.push(dist);
                // println!("Point_1: {}, {}, Point_2: {}, {}, Dist: {}", curr_point.0, curr_point.1, point.0, point.1, dist);
            }
        }
    }

    // println!("{:?}", tile_count_heap);
    tile_count_heap.pop().unwrap()
}