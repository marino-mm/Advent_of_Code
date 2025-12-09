use std::{collections::BTreeSet, fs::File, io::{self, BufWriter, Write}, time::Instant, u64};

fn main() {
    let start = Instant::now();
    let data = include_str!("input.txt");
    // let data = include_str!("test.txt");

    let res = solve(&data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result: {:?}", res);

}

fn solve(data: &str) -> u64 {

    // let mut tile_count_heap = BinaryHeap::<u64>::new();
    let mut points = BTreeSet::<(u64, u64)>::new();

    let mut x_min: u64 = u64::MAX;
    let mut x_max: u64 = u64::MIN;

    let mut y_min: u64 = u64::MAX;
    let mut y_max: u64 = u64::MIN;

    for line in data.lines(){
        let parts:Vec<u64> = 
            line
            .split(',')
            .map(|p| p.parse::<u64>().unwrap())
            .collect();
        let x = *parts.get(0).unwrap();
        let y = *parts.get(1).unwrap();
        let curr_point = (x, y);
        points.insert(curr_point);

        x_min = x_min.min(x);
        x_max = x_max.max(x);

        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }

    let _ = write_to_file(&points, x_max, x_min, y_max, y_min);

    1

}

fn write_to_file(points: &BTreeSet::<(u64, u64)>, x_max: u64, x_min: u64, y_max: u64, y_min: u64) -> io::Result<()>{
    let file = File::create("vis.txt")?;
    let mut write_buff = BufWriter::new(file);

    println!("Number of cells: {}\n", (x_max - x_min) * (y_max - y_min));

    let mut i:u128 = 1;

    for y in y_min..=y_max {
        let mut line = String::new();
        for x in x_min..=x_max {
            if points.contains(&(x, y)){
                line.push('X');
            }else{
                line.push('.');
            }
            i += 1;
            if i % 100_000 == 0{
                print!(" Line: {}\r", i);
            }
        }
        write_buff.write(line.as_bytes())?;
        write_buff.write_all(b"\n")?;
    }
    write_buff.flush()?;
    Ok(())
}