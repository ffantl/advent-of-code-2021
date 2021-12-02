use std::fs::File;
use std::path::Path;
use std::env;
use std::io::BufRead;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    if let Ok(lines) = read_lines(filename) {
        let result = dowork(lines);
        println!("{}", result)
    }
}

fn dowork(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut total_count: i32 = 0;
    let mut vec: Vec<i32> = Vec::new();
    let mut previous_sum: i32 = 0;
    


    for line in lines {
        if let Ok(str_depth) = line {
            if vec.len() == 3 {
                vec.remove(0);
            }

            let depth = str_depth.parse::<i32>().unwrap();
            vec.push(depth);
            
            if vec.len() == 3 {
                let temp_sum = sumdepths(vec[0], vec[1], vec[2]);
                
                if previous_sum > 0 && previous_sum < temp_sum {
                    total_count += 1
                }

                previous_sum = temp_sum
            }
        }
    }

    total_count
}

fn sumdepths(a: i32, b:i32, c: i32) -> i32 {
    a + b + c
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    filename
}