use std::time::Instant;

const INPUT: &[u8] = include_bytes!("input.txt");

const MAX: u32 = 100;
const BIG_ENOUGH_MAX_MULTIPLE: u32 = MAX * MAX;
const INITIAL_DIAL: u32 = 50;
const R: u8 = b'R';
const NEWLINE: u8 = b'\n';

fn main() {
    let _perf_start_timer = Instant::now();

    let mut dial: u32 = INITIAL_DIAL;
    let mut zero_count: u32 = 0;
    INPUT.split(|&b| b == NEWLINE).for_each(|line| unsafe {
        let val: u32 = std::str::from_utf8_unchecked(&line[1..]).parse().unwrap();

        dial = if line[0] == R {
            (dial + val) % MAX
        } else {
            (dial + BIG_ENOUGH_MAX_MULTIPLE - val) % MAX
        };
        zero_count += if dial == 0 { 1 } else { 0 };
    });

    let _perf_duration = _perf_start_timer.elapsed();
    println!("Time elapsed: {:?}", _perf_duration);
    println!("Time elapsed: {:?}ms", _perf_duration.as_secs_f64() * 1000.0);

    println!("Zero count: {}", zero_count);
}
