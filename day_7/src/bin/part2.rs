use std::process::exit;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    // let data = include_bytes!("test.txt");
    let data = include_bytes!("input.txt");
    let res = solve(data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result Solved: {}", res);
}


fn solve(data: &[u8]) -> u64 {
    let mut res = 0;

    let mut line_size = 0;
    for sample in data.windows(2) {
        if sample != b"\r\n"{
            line_size += 1;
        }else { break; }
    }

    let line_count = data.len() / (line_size + 2);

    let mut curr_line_buffer: Vec<u64> = vec![0; line_size];
    let mut new_line_buffer: Vec<u64> = vec![0; line_size];

    for (line_i, line) in data.chunks(line_size +2).enumerate() {
        if line_i % 2 == 0 {
            for ind in 0..line_size {
                let c = line[ind];
                if c == b'S'{
                    new_line_buffer[ind] = 1;
                }
                else if c == b'^' && curr_line_buffer[ind] > 0{
                    new_line_buffer[ind - 1] =  curr_line_buffer[ind] + new_line_buffer[ind - 1];
                    new_line_buffer[ind + 1] =  curr_line_buffer[ind] + new_line_buffer[ind + 1];
                    if new_line_buffer[ind] == 0 {
                        new_line_buffer[ind] = 0;
                    }
                }else if curr_line_buffer[ind] > 0{
                    new_line_buffer[ind] = curr_line_buffer[ind] + new_line_buffer[ind];
                }
            }
            curr_line_buffer = new_line_buffer.clone();
            new_line_buffer = vec![0; line_size];
        }
    }
    for &numb in curr_line_buffer.iter(){
        if numb != 0{
            res += numb;
        }
    }
    res
}