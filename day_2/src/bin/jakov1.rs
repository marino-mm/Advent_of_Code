use std::time::Instant;

const INPUT: &[u8] = include_bytes!("intput.txt");

const COMMA: u8 = b',';
const DASH: u8 = b'-';

const DIVISORS: &[u64] = &[0, 10, 100, 1000, 10000, 100000, 1000000];
const MIN_DIGIT_REPEATING: &[u64] = &[
    0,
    11,
    0,
    1010,
    0,
    100100,
    0,
    10001000,
    0,
    1000010000,
    0,
    100000100000,
    0,
    10000001000000,
];
const MAX_DIGIT_REPEATING: &[u64] = &[
    0,
    0,
    0,
    99,
    0,
    9999,
    0,
    999999,
    0,
    99999999,
    0,
    9999999999,
    0,
    999999999999,
];

fn main() {
    let _perf_start_timer = Instant::now();
    let mut sum: u64 = 0;

    INPUT.split(|&b| b == COMMA).for_each(|line: &[u8]| unsafe {
        let (left, right) = line.split_at_unchecked(line.iter().position(|&b| b == DASH).unwrap());
        let mut left_num: u64 = std::str::from_utf8_unchecked(left).parse().unwrap();
        let mut right_num: u64 = std::str::from_utf8_unchecked(&right[1..]).parse().unwrap();
        let mut left_len = left.len();
        let right_len = right.len() - 1;
        if (left_len != right_len) || (left_len & 1 == 0) {
            // bound left
            if left_len & 1 != 0 {
                left_num = MIN_DIGIT_REPEATING[left_len];
                left_len += 1;
            }

            // bound right
            if right_len & 1 != 0 {
                right_num = MAX_DIGIT_REPEATING[right_len];
                // right_len -= 1; -- unused afterwards
            }

            let half_len = left_len / 2;
            let divisor = DIVISORS[half_len];

            let left_top = left_num / divisor;
            let left_bottom = left_num % divisor;
            let right_top = right_num / divisor;
            let right_bottom = right_num % divisor;

            let start = left_top + (left_top < left_bottom) as u64;
            let end = right_top - (right_top > right_bottom) as u64;

            sum += ((divisor + 1) * (end - start + 1) * (start + end)) / 2;
        }
    });

    let _perf_duration = _perf_start_timer.elapsed();
    println!("Time elapsed: {:?}", _perf_duration);

    println!("Sum: {}", sum);
}