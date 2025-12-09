use std::{time::Instant, u64};

fn main() {
    let start = Instant::now();
    // let data = include_str!("input.txt");
    let data = include_str!("test.txt");

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

    for point_a in &points{
        for point_b in &points{
            if point_a != point_b {
                if tile_count_max < tile_count(point_a, point_b){
                    if is_valid_tile(point_a, point_b, &points){
                        tile_count_max = tile_count_max.max(tile_count(point_a, point_b));
                        println!("Point_a: {:?}, Point_b: {:?}, size: {:?}", point_a, point_b, tile_count_max);

                    }
                }
            }
        }
    }

    tile_count_max
}
fn tile_count(&point_a: &(u64, u64), &point_b: &(u64, u64)) -> u64 {
    (point_a.0.abs_diff(point_b.0) + 1) * (point_a.1.abs_diff(point_b.1) + 1)
}

fn is_valid_tile(point_a: &(u64, u64), point_b: &(u64, u64), points: &Vec::<(u64, u64)>) -> bool{

    let max_x = point_a.0.max(point_b.0);
    let max_y = point_a.1.max(point_b.1);

    let min_x = point_a.0.min(point_b.0);
    let min_y = point_a.1.min(point_b.1);

    let alt_a = (point_a.0, point_b.1);
    let alt_b = (point_b.0, point_a.1);

    if !point_in_polygon(&alt_a, points) || !point_in_polygon(&alt_b, points){
        return false;
    }

    for point in points{
        if point != point_a && point != point_b{
            if min_x < point.0 && point.0 < max_x && min_y < point.1 && point.1 < max_y{
                return false;
            }
        }
    }
    true
}

fn point_in_polygon(target_point:&(u64, u64), points: &Vec<(u64, u64)>) -> bool {
    let mut intersections = 0;
    let n = points.len();
    let (x, y) = *target_point;

    if n < 3 { return false; }

    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];

        let (x1, y1) = p1;
        let (x2, y2) = p2;

        // Ensure the ray's y is strictly between the segment's y-coordinates.
        // This is the most common way to handle edge cases at vertices correctly.
        let crosses_upward = (y1 <= y) && (y2 > y);
        let crosses_downward = (y1 > y) && (y2 <= y);

        if crosses_upward || crosses_downward {
            // Check if the intersection point's x is to the right of the ray's start (target_point's x).
            // Formula rearranged to cross-product for integer math:
            // x < x1 + (x2 - x1) * (y - y1) / (y2 - y1)
            // x * (y2 - y1) < x1 * (y2 - y1) + (x2 - x1) * (y - y1)

            // Note: We use i64 subtraction and multiplication.
            let dx = x2 as i64 - x1 as i64;
            let dy = y2 as i64 - y1 as i64;
            let target_dy = y as i64 - y1 as i64;

            // x * dy < x1 * dy + dx * target_dy
            // (x - x1) * dy < dx * target_dy
            if (x as i64 - x1 as i64) * dy < dx * target_dy {
                // If dy > 0, we check for a standard crossing (ray to the right)
                // If dy < 0, the inequality flips (because we multiplied by a negative number),
                // but the logic here handles it correctly by looking at the sign of the cross-product difference.
                intersections += 1;
            }
        }
    }

    // Odd number of intersections means the point is inside.
    intersections % 2 != 0
}