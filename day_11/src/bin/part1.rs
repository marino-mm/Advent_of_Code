use std::{collections::{HashMap, VecDeque}, time::Instant};

fn main() {
    let start = Instant::now();
    let data = include_str!("input.txt");
    // let data = include_str!("test.txt");

    let res = solve(&data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result: {:?}", res);
}

fn solve(data: &str) -> u64{

    let mut res = 0;
    let mut device_relations:HashMap<String, Vec<String>> = HashMap::new();

    for line in data.lines(){
        let parts:Vec<&str> = line.split_whitespace().collect();
        let start = parts[0].replace(":", "");

        let end_values: Vec<String> = parts[1..]
            .iter() 
            .map(|s| s.to_string())
            .collect();
            device_relations.insert(start, end_values);
    }

    res = find_all_paths(&device_relations);
    res
}

fn find_all_paths(device_relations: &HashMap<String, Vec<String>>) -> u64 {
    let mut count = 0;
    let start = "you".to_string();
    let end = "out".to_string();
    let mut heap = VecDeque::<String>::new();

    device_relations.get(&start)
                    .unwrap()
                    .iter()
                    .for_each(|n|{
                        heap.push_back(n.clone());
                    });

    loop {
        match heap.pop_front(){
            Some(name) => {

                for node in device_relations.get(&name).unwrap(){
                    if node == &end{
                        count += 1;
                    }else {
                        heap.push_front(node.clone());
                    }
                }
            },
            None => {return count;},
        }
    }
}