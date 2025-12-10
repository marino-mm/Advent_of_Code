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
    
    let mut max_tile_count = u64::MIN;
    let mut points = Vec::<(u64, u64)>::new();

    for line in data.lines() {
        let parts: Vec<u64> = line.split(',').map(|p| p.parse::<u64>().unwrap()).collect();
        let curr_point = (*parts.get(0).unwrap(), *parts.get(1).unwrap());
        points.push(curr_point);
    }

    
    for point_a in &points {
        for point_b in &points {
            if point_a != point_b {
                let curr_tile_count = count_tiles(point_a, point_b);
                if curr_tile_count > max_tile_count {
                    if is_valid_polygon(point_a, point_b, &points) {
                        max_tile_count = curr_tile_count;
                        println!("A: {:?},B: {:?}, dis: {}", point_a, point_b, max_tile_count);
                    }
                }
            }
        }
    }
    

    max_tile_count
}

fn count_tiles(point_a: &(u64, u64), point_b: &(u64, u64)) -> u64 {
    let x_len = (point_a.0).abs_diff(point_b.0) + 1;
    let y_len = (point_a.1).abs_diff(point_b.1) + 1;
    x_len * y_len
}

fn is_valid_polygon(point_a: &(u64, u64), point_b: &(u64, u64), points: &Vec<(u64, u64)>) -> bool {
    let alt_point_a = (point_a.0, point_b.1);
    let alt_point_b = (point_b.0, point_a.1);

    if point_is_polygon(alt_point_a, points) {
        if point_is_polygon(alt_point_b, points) {
            if !area_hase_deadspace(point_a, point_b, points) {
                return true;
            }
        }
    }
    false
}

fn point_is_polygon(point: (u64, u64), points: &Vec<(u64, u64)>) -> bool {
    let (px, py) = point;
    let mut count = 0; // Intersection count

    for i in 0..points.len() {
        let (x_a, y_a) = points[i];
        let (x_b, y_b) = points[(i + 1) % points.len()];

        let (min_x, max_x) = (x_a.min(x_b), x_a.max(x_b));
        let (min_y, max_y) = (y_a.min(y_b), y_a.max(y_b));
        
        if x_a == x_b {
            if px == x_a && py >= min_y && py <= max_y {
                return true;
            }
        } 
        else { 
            if py == y_a && px >= min_x && px <= max_x {
                return true;
            }
        }
        if x_a == x_b && x_a >= px { 
            if py >= min_y && py < max_y {
                count += 1;
            }
        }
    }
    count % 2 != 0
}

fn area_hase_deadspace(point_a: &(u64, u64),point_b: &(u64, u64),points: &Vec<(u64, u64)>,) -> bool {
    let alt_point_a = (point_a.0, point_b.1);
    let alt_point_b = (point_b.0, point_a.1);

    let test_polygon = vec![point_a, &alt_point_a, &point_b, &alt_point_b];

    for i in 0..points.len() {
        let point_a = points.get(i).unwrap();
        let point_b = points.get((i + 1) % points.len()).unwrap();

        let edge = (*point_a, *point_b);

        for j in 0..test_polygon.len() {
            let test_point_a = *test_polygon.get(j).unwrap();
            let test_point_b = *test_polygon.get((j + 1) % test_polygon.len()).unwrap();
            let test_edge = (*test_point_a, *test_point_b);
            if edges_cross(edge, test_edge) {
                return true;
            }
        }
    }
    false
}

fn edges_cross(edge_a: ((u64, u64), (u64, u64)), edge_b: ((u64, u64), (u64, u64))) -> bool {
    let edge_a_horisontal = edge_a.0.1 == edge_a.1.1;
    let edge_b_horisontal = edge_b.0.1 == edge_b.1.1;

    let horisontal_edge;
    let vertical_edge;
    if edge_a_horisontal != edge_b_horisontal {
        if edge_a_horisontal {
            horisontal_edge = edge_a;
            vertical_edge = edge_b;
        } else {
            horisontal_edge = edge_b;
            vertical_edge = edge_a;
        }

        let vertical_edge_min_y = vertical_edge.0.1.min(vertical_edge.1.1);
        let vertical_edge_max_y = vertical_edge.0.1.max(vertical_edge.1.1);

        let horisontal_edge_min_x = horisontal_edge.0.0.min(horisontal_edge.1.0);
        let horisontal_edge_max_x = horisontal_edge.0.0.max(horisontal_edge.1.0);
        if horisontal_edge_min_x < vertical_edge.0.0 && vertical_edge.0.0 < horisontal_edge_max_x {
            if vertical_edge_min_y < horisontal_edge.0.1 && horisontal_edge.0.1 < vertical_edge_max_y{
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edge(p1: (u64, u64), p2: (u64, u64)) -> ((u64, u64), (u64, u64)) {
        (p1, p2)
    }

    #[test]
    fn point_is_polygon_test() {
        let points: Vec<(u64, u64)> = vec![(5, 5), (10, 5), (10, 10), (5, 10)];
        let test_point = (1, 1);
        assert_eq!(point_is_polygon(test_point, &points), false);
    }
    #[test]
    fn point_is_polygon_test_2() {
        let points: Vec<(u64, u64)> = vec![(5, 5), (10, 5), (10, 10), (5, 10)];
        let test_point = (6, 6);
        assert_eq!(point_is_polygon(test_point, &points), true);
    }

    #[test]
    fn point_is_polygon_test_3() {
        let points: Vec<(u64, u64)> = vec![(5, 5), (10, 5), (10, 10), (5, 10)];
        let test_point = (5, 5);
        assert_eq!(point_is_polygon(test_point, &points), false);
    }

    #[test]
    fn point_is_polygon_test_4() {
        let points: Vec<(u64, u64)> = vec![(5, 5), (10, 5), (10, 10), (5, 10)];
        let test_point = (10, 10);
        assert_eq!(point_is_polygon(test_point, &points), false);
    }

    #[test]
    fn test_1_orthogonal_cross() {
        let edge_a = edge((5, 0), (5, 10));
        let edge_b = edge((0, 5), (10, 5));
        assert_eq!(
            edges_cross(edge_a, edge_b),
            true,
            "Test 1 Failed: Should cross."
        );
    }

    #[test]
    fn test_2_parallel_separated() {
        let edge_a = edge((2, 0), (2, 5));
        let edge_b = edge((4, 0), (4, 5));
        assert_eq!(
            edges_cross(edge_a, edge_b),
            false,
            "Test 2 Failed: Should not cross (separated)."
        );
    }

    #[test]
    fn test_3_parallel_overlap() {
        let edge_a = edge((5, 0), (5, 5));
        let edge_b = edge((5, 3), (5, 8));
        assert_eq!(
            edges_cross(edge_a, edge_b),
            false,
            "Test 3 Failed: Should cross (overlap)."
        );
    }

    #[test]
    fn test_4_orthogonal_miss() {
        let edge_a = edge((5, 0), (5, 2));
        let edge_b = edge((0, 5), (10, 5));
        assert_eq!(
            edges_cross(edge_a, edge_b),
            false,
            "Test 4 Failed: Should not cross (missed)."
        );
    }

    #[test]
    fn test_5_endpoint_touch() {
        let edge_a = edge((5, 0), (5, 5));
        let edge_b = edge((5, 5), (10, 5));
        assert_eq!(edges_cross(edge_a, edge_b),false,"Test 5 Failed: Should cross (endpoint touch)."
        );
    }
    #[test]
    fn test_1_is_valid_polygon() {
        let edge_a = edge((81134, 87579), (16655, 14097));
        let edge_b = edge((5, 5), (10, 5));
        assert_eq!(edges_cross(edge_a, edge_b),false,"Test 5 Failed: Should cross (endpoint touch)."
        );
    }
    #[test]
    fn test_1_has_dead_space(){
        let test_points = vec![(1723,50049),(94523,50049),(94523,48719),(1723,48719)];
        let test_point_a = (15675, 84247);
        let test_point_b = (83843, 16232);

        assert_eq!(area_hase_deadspace(&test_point_a, &test_point_b, &test_points),true);
    }
}
