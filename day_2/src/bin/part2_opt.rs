use std::{fs::read_to_string, thread, time::Instant};


fn invalid_number(numb: &u64) -> bool {
    let numb_str = numb.to_string();
    let str_len = numb_str.len();

    for pattern_len in 1..=str_len /2 {
        let replace_pattern = &numb_str[0..pattern_len];
        if str_len % pattern_len != 0 {
            continue;
        }
        //if numb_str.replace(&replace_pattern, "").len() == 0 {
        let match_str = replace_pattern.repeat(str_len / pattern_len);
        if match_str == numb_str{
            return true;
        }
    }

    false
}

fn solve() -> u64 {
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let input = read_to_string(r"src\bin\intput.txt").unwrap();

    let mut handles = vec![];

    for range in input.split(","){
        let range = range.to_string();
        let handle = thread::spawn(move || {
            let mut res = 0;
            let range_parts: Vec<&str> = range.trim().split("-").collect();
            for numb in range_parts[0].parse::<u64>().expect("Cant convert to number") ..=range_parts[1].parse::<u64>().expect("Cant convert to number"){
                if invalid_number(&numb){
                    res += numb;
                    }
                }
            res
        });
        handles.push(handle);
    }


    let mut outputs = vec![];
    for handle in handles {
        match handle.join() {
            Ok(output) => outputs.push(output),
            Err(e) => eprintln!("A thread panicked: {:?}", e),
        }
    }
    outputs.iter().sum()
}

fn main() {
    let start = Instant::now();

    let res = solve();
    let duration = start.elapsed();

    println!("Result {}", res);
    println!("Duration {:?}", duration);
}
