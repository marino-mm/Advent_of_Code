use std::time::Instant;

const INPUT: &[u8] = include_bytes!("intput.txt");

const COMMA: u8 = b',';
const DASH: u8 = b'-';

const PRIMES: &[&[u64]] = &[
    &[1],
    &[1],
    &[1],
    &[1],
    &[2, 1],
    &[1],
    &[3, 2, 1], // 3, then 2
    &[1],
    &[4, 1],
    &[3, 1],
    &[5, 2, 1], // 5, then 2
    &[1],
];
const DIVISORS: &[u64] = &[
    0, 1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];
const ONES: &[u64] = &[
    0,
    1,
    11,
    111,
    1111,
    11111,
    111111,
    1111111,
    11111111,
    111111111,
    1111111111,
    11111111111,
];
const MIN_DIGIT_REPEATING: &[u64] = &[
    0,
    0,
    11,
    111,
    1010,
    11111,
    100100, // 3 > 2
    1111111,
    10001000,
    100100100,
    1000010000, // 5 > 2
    11111111111,
];
const MAX_DIGIT_REPEATING: &[u64] = &[
    0,
    9,
    99,
    999,
    9999,
    99999,
    999999,
    9999999,
    99999999,
    999999999,
    9999999999,
    99999999999,
    999999999999,
];

fn main() {
    let _perf_start_timer = Instant::now();
    let mut sum: u64 = 0;

    INPUT.split(|&b| b == COMMA).for_each(|line: &[u8]| unsafe {
        let (fst, snd) = line.split_at_unchecked(line.iter().position(|&b| b == DASH).unwrap());
        let mut left_num: u64 = std::str::from_utf8_unchecked(fst).parse().unwrap();
        let right_num: u64 = std::str::from_utf8_unchecked(&snd[1..]).parse().unwrap();
        let mut left_len = fst.len();
        let right_len = snd.len() - 1;

        if left_len == 1 {
            left_num = 11;
            left_len = 2;
        }

        for ln in left_len..=right_len {
            let left = if ln == left_len {
                left_num
            } else {
                MIN_DIGIT_REPEATING[ln]
            };

            let right = if ln == right_len {
                right_num
            } else {
                MAX_DIGIT_REPEATING[ln]
            };

            let primes: &[u64] = PRIMES[ln];
            for prime in primes {
                let period: usize = *prime as usize;

                let left_bound = next_repeating(left, period, ln);
                let right_bound = prev_repeating(right, period, ln);
                if left_bound <= right_bound {
                    let ones = count_ones(left_bound, right_bound, ln);
                    if period == 1 {
                        sum += ones;
                    } else {
                        let period_divisor = DIVISORS[period + 1];
                        let left_period = left_bound % period_divisor;
                        let right_period = right_bound % period_divisor;

                        let range_sum = sum_range(left_period, right_period);
                        let mut m = 1;
                        for _ in 0..(ln / period) {
                            sum += m * range_sum;
                            m *= period_divisor;
                        }
                        sum -= ones;
                    }
                }
            }
        }
    });

    let _perf_duration = _perf_start_timer.elapsed();
    println!("Time elapsed: {:?}", _perf_duration);

    println!("Sum: {}", sum);
}

#[inline]
fn count_ones(start: u64, end: u64, len: usize) -> u64 {
    let ones = ONES[len];
    let divisor = DIVISORS[len];
    let start_first_digit = start / divisor;
    let end_first_digit = end / divisor;

    let start_first_digit = if start_first_digit * ones >= start {
        start_first_digit
    } else {
        start_first_digit + 1
    };

    let end_first_digit = if end_first_digit * ones <= end {
        end_first_digit
    } else {
        end_first_digit - 1
    };

    if end_first_digit < start_first_digit {
        0
    } else {
        ones * sum_range(start_first_digit, end_first_digit)
    }
}

#[inline(always)]
fn sum_range(from: u64, to: u64) -> u64 {
    ((to - from + 1) * (from + to)) / 2
}

#[inline(always)]
fn extract_period(n: u64, period: usize, period_i: usize, len: usize) -> u64 {
    let shift = len - (period_i + 1) * period;
    let divisor = DIVISORS[shift + 1];
    let modulo = DIVISORS[period + 1];
    (n / divisor) % modulo
}

#[inline]
fn build_repeating(pattern: u64, period: usize, reps: usize) -> u64 {
    let mut result = 0;
    let mut multiplier = 1;
    let period_divisor = DIVISORS[period + 1];

    for _ in 0..reps {
        result += pattern * multiplier;
        multiplier *= period_divisor;
    }
    result
}

#[inline]
fn next_repeating(start: u64, period: usize, len: usize) -> u64 {
    if period == 1 {
        return start;
    }

    let num_repeats = len / period;

    for i in 0..(num_repeats - 1) {
        let current = extract_period(start, period, i, len);
        let next = extract_period(start, period, i + 1, len);

        if next < current {
            return build_repeating(current, period, num_repeats);
        } else if next > current {
            return build_repeating(current + 1, period, num_repeats);
        }
    }

    start
}

#[inline]
fn prev_repeating(end: u64, period: usize, len: usize) -> u64 {
    if period == 1 {
        return end;
    }

    let num_repeats = len / period;

    for i in 0..(num_repeats - 1) {
        let current = extract_period(end, period, i, len);
        let next = extract_period(end, period, i + 1, len);

        if current < next {
            return build_repeating(current, period, num_repeats);
        } else if current > next {
            return build_repeating(current - 1, period, num_repeats);
        }
    }

    end
}