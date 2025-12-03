use std::time::Instant;


fn find_biggest_number(list: &str) -> (u64, u64){

    let mut bn = u64::MIN;
    let mut skipps:u64 = 0;
    for (i, c) in list.chars().enumerate(){
        let c = c.to_string().parse::<u64>().unwrap();
        if c == 9u64{
            return (c, i as u64);
        } else if bn < c {
            bn = c;
            skipps = i as u64;
        }
    }
    (bn, skipps)
}

fn solve(data: &str) -> u64 {
    let mut total = 0;
    let target_numb = 11;
    for bank in data.lines() {
        let mut skipps = (bank.len() - target_numb) as u64;
        let mut new_numb = Vec::<u64>::new();
        let mut bank_index = 0;
        while skipps > 0 && new_numb.len() < 12 {
            let res = find_biggest_number(&bank[bank_index as usize..bank_index + skipps as usize]);
            new_numb.push(res.0);
            skipps -= res.1;
            bank_index += res.1 as usize + 1;
        }
        let biggest_numb: String = new_numb.iter().map(|n| n.to_string()).collect();
        total += biggest_numb.parse::<u64>().unwrap();
    }
    total
}

fn main() {
    let data = include_str!("./input.txt");
    // let data = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    // let data = "811111111111119";

    let start = Instant::now();
    let result = solve(&data);
    let duration = start.elapsed();
    println!("Result: {:?}", result);
    println!("Time elapsed: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case1() {
        assert_eq!(solve("987654321111111"), 987654321111);
    }

    #[test]
    fn test_example_case2() {
        assert_eq!(solve("811111111111119"), 811111111119);
    }

    #[test]
    fn test_example_case3() {
        assert_eq!(solve("234234234234278"), 434234234278);
    }

    #[test]
    fn test_example_case4() {
        assert_eq!(solve("818181911112111"), 888911112111);
    }
}

