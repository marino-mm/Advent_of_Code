use std::time::Instant;

fn main() {

    let data = include_str!("input.txt");
    /*
    let data = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
     */
    let start = Instant::now();
    let res = solve(&data);
    let duration = start.elapsed();
    println!("Solve time: {:?}", duration);
    println!("Result: {}", res);
}

fn solve(data: &str) -> u64 {
    let mut problems = Vec::<Vec<u64>>::new();
    let mut res = 0;
    for (line_i, line) in data.lines().enumerate() {
        for (number_i, number) in line.split_whitespace().enumerate() {
            if line_i == 0{
                let parsed_number = number.parse::<u64>().unwrap();
                problems.push(vec![parsed_number]);
            } else {
                if number.chars().nth(0).unwrap().is_numeric(){
                    problems[number_i].push(number.parse::<u64>().unwrap());
                } else {
                    let char = number.chars().nth(0).unwrap();
                    if char == '+'{
                        let sum: u64 = problems[number_i].iter().sum();
                        res += sum;
                    }
                    else {
                        let mut sum = 1;
                        problems[number_i].iter().for_each(
                            |n|
                                sum = sum * n
                        );
                        res += sum;
                    }
                }
            }
        }
    }
    res
}
