use std::time::Instant;


fn split_bytes(){
    let data = include_bytes!("./input.txt");
    let _bank_list:Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
}

fn split_str(){
    let data = include_str!("./input.txt");
    let _bank_list:Vec<&str> = data.split("\n").collect();
}

fn main() {
    let start = Instant::now();

    split_bytes();
    // split_str();

    let duration = start.elapsed();
    // println!("Result: {:?}", data);
    println!("Time elapsed: {:?}", duration);
}