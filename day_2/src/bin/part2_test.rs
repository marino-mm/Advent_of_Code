use std::{fs::read_to_string, time::Instant};

fn invalid_number(numb: &u64) -> (bool, usize) {
    let numb_str = numb.to_string();
    let str_len = numb_str.len();


    for pattern_len in 1..str_len {
        let replace_pattern = &numb_str[0..pattern_len];
        if str_len % pattern_len != 0 {
            continue;
        }
        if numb_str.replace(&replace_pattern, "").len() == 0 {
            return (true, pattern_len);
        }
    }

    (false, 0)
}

fn solve() -> u64 {
    let start = Instant::now();
    let mut res = 0;
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let input = read_to_string(r"src\bin\intput.txt").unwrap();

    let duration_file_load = start.elapsed();
    let start_loop = Instant::now();

    for range in input.split(",") {
        let range_parts: Vec<&str> = range.trim().split("-").collect();
        let start = range_parts[0].parse::<u64>().expect("Cant convert to number");
        let end = range_parts[1].parse::<u64>().expect("Cant convert to number");
        // for numb in start..=end
        let mut numb = start;
        while numb <= end 
        {
            let fun_res =  invalid_number(&numb);
            if fun_res.0 {
                res += numb;
                // numb += 10_u64.pow(fun_res.1 as u32 -1 );
                numb += 1;
            } else {
                numb += 1;
            }
        }
    }
    let duration_loop = start_loop.elapsed();

    println!("\nFile load duration {:?}", duration_file_load);
    println!("Loop duration {:?}\n", duration_loop);
    res
}

fn main() {
    let start = Instant::now();
    let res = solve();
    let duration = start.elapsed();

    println!("Result {}", res);
    println!("Duration {:?}", duration);
}
