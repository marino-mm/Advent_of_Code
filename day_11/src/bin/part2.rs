use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

fn main() {
    let start = Instant::now();
    let data = include_str!("input.txt");
    // let data = include_str!("test2.txt");

    let res = solve(&data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result: {:?}", res);
}

fn solve(data: &str) -> u64 {
    let mut device_relations: HashMap<String, Vec<String>> = HashMap::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let start = parts[0].replace(":", "");

        let end_values: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        device_relations.insert(start, end_values);
    }

    // svr -> dac -> fft -> out
    // svr -> fft -> dac -> out
    let mut first_path = find_all_paths_dp(&device_relations, "svr".to_string(), "dac".to_string());
    first_path = first_path * find_all_paths_dp(&device_relations, "dac".to_string(), "fft".to_string());
    first_path = first_path * find_all_paths_dp(&device_relations, "fft".to_string(), "out".to_string());

    let mut second_path = find_all_paths_dp(&device_relations, "svr".to_string(), "fft".to_string());
    second_path = second_path * find_all_paths_dp(&device_relations, "fft".to_string(), "dac".to_string());
    second_path = second_path * find_all_paths_dp(&device_relations, "dac".to_string(), "out".to_string());
    
    first_path.max(second_path)
}

fn find_all_paths_dp(
    device_relations: &HashMap<String, Vec<String>>,
    start: String,
    end: String,
) -> u64 {
    let mut memo: HashMap<String, u64> = HashMap::new();
    count_paths_to_end(device_relations, &start, &end, &mut memo)
}

fn count_paths_to_end(
    device_relations: &HashMap<String, Vec<String>>,
    current: &String,
    end: &String,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(count) = memo.get(current) {
        return *count;
    }

    if current == end {
        return 1;
    }

    let mut total_paths = 0;
    if let Some(neighbors) = device_relations.get(current) {
        for neighbor in neighbors {
            total_paths += count_paths_to_end(device_relations, neighbor, end, memo);
        }
    }
    memo.insert(current.to_string(), total_paths);
    total_paths
}
