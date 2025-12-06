use std::time::Instant;

fn main() {

    let data = include_bytes!("input.txt");
    // let data = include_bytes!("test.txt");

    let start = Instant::now();
    let res = solve(data);
    let duration = start.elapsed();
    println!("Solve time: {:?}", duration);
    println!("Result: {}", res);
}

fn solve(data: &[u8; 18628]) -> u64 {
    let mut res = 0;

    let mut line_len = 1;
    for &c in data {
        if c == b'\n' {
            break;
        } else {
            line_len += 1;
        }
    }
    let line_count = (data.len() as f64 / line_len as f64).ceil() as usize;
    let data_len = data.len() -1;

    let mut numbs: Vec<u64> = Vec::new();
    for col_i in 0..line_len -2 {
        let mut numb = 0;
        let mut func = "";

        let mut pow =0;

        for line_i in 0..line_count{
            let char_i = data_len - (line_i * line_len + col_i);
            let c = data[char_i];
            if (c as char).is_numeric() || c == b' '{
                if c == b' ' || c == b'0'{
                    continue;
                } else{
                    numb += (c - 48) as u64 * (10u64.pow(pow as u32));
                    pow +=1;
                }
            }
            if c as char == '*'{
                func = "*";
            } else if c as char == '+' {
                func = "+";
            }
        }
        if numb != 0 {
            numbs.push(numb);
        }
        if func != ""{
            if func == "+" {
                res += numbs.iter().sum::<u64>();
            } else {
                res += numbs.iter().product::<u64>();
            }
            numbs.clear();
        }
    }
    res
}
