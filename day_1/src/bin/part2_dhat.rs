use std::{env, fs::read_to_string, time::Instant};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn task_2(instruction_list: &Vec<String>, starting_pos: i32) -> i32 {
    let mut zero_count = 0;
    let mut pos = starting_pos;
    for instruction in instruction_list.iter() {
        let direction = instruction[0..1].to_string();
        let amount = instruction[1..]
            .parse::<i32>()
            .expect("Failed to convert instruction to integer");
        let mut new_pos: i32;
        if direction == "L" {
            new_pos = pos - amount;
            zero_count += new_pos.abs() / 100;
            if new_pos < 0 && pos != 0{
                zero_count += 1
            }
        } else {
            new_pos = pos + amount;
            zero_count += new_pos / 100;
        }

        if new_pos == 0{
            zero_count += 1
        }

        new_pos = new_pos % 100;
        if new_pos <0 {
            new_pos = 100 + new_pos;
        }
        pos = new_pos;
    }
    zero_count
}

fn get_file_data() -> Vec<String> {
    let project_dir =  env::current_dir().expect("Failed to load project dir path!");
    let file_dir = "src/bin/input.txt";
    let file_data: Vec<String> = read_to_string(project_dir.join(file_dir))
        .expect("Can't find file")
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect();
    file_data
}

#[allow(dead_code)]
fn get_test_data() -> Vec<String> {
    let test_data: Vec<String> = vec!["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",]
        .iter()
        .map(|s| s.trim().to_string())
        .collect();
    test_data
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    
    let start = Instant::now();

    // let instruction_data = get_test_data();
    let instruction_data = get_file_data();
    let numb = 50;
    let res = task_2(&instruction_data, numb);
    let duration = start.elapsed().as_secs_f32();
    println!("Task duration: {}s", duration);
    println!("Task duration: {}ms", (duration * 10e3f32));
    println!("Task duration: {}µs", (duration * 10e6f32));
    println!("Task duration: {}ns", (duration * 10e9f32));
    println!("Task duration: {}ps", (duration * 10e12f32));
    
    println!("\nReslut: {}", res);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_1(){
        let test_data: Vec<String> = vec!["L200"]
        .iter()
        .map(|s| s.trim().to_string())
        .collect();
        assert_eq!(task_2(&test_data, 1), 2)
    }
}