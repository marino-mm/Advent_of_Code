use std::time::Instant;

fn solve(data: &str) -> u16 {
    let mut total = 0;

    for bank in data.lines() {
        let mut first_digit = bank.chars().nth(0).unwrap().to_string().parse::<u16>().unwrap();
        let mut second_digit = bank.chars().nth(1).unwrap().to_string().parse::<u16>().unwrap();

        let mut char_index = 1;
        while char_index < bank.len() {
            let digit = bank.chars().nth(char_index).unwrap().to_string().parse::<u16>().unwrap();
            if digit > first_digit && char_index < bank.len() - 1 {
                first_digit = digit;
                second_digit = 0;
            } else if digit > second_digit {
                second_digit = digit;
            }
            char_index += 1;
        }
        total += 10 * first_digit + second_digit;
    }
    total
}

fn main() {
    let data = include_str!("./input.txt");

    let start = Instant::now();
    let result = solve(&data);
    let duration = start.elapsed();
    println!("Result: {}", result);
    println!("Time elapsed: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(solve("987654321111111\n811111111111119\n234234234234278\n818181911112111"),357);
        assert_eq!(solve("2442434433134612455623233234322444122344445125234325442255354144222442234851445124211423212442232243"),99);
    }
}
