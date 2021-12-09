use std::env;
use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    if let Ok(lines) = read_lines(filename) {
        let result = dowork(lines);
        println!("{}", result)
    }
}

// 10 unique values
// 1, 4, 7, 8 are unique due to the number of segments
// how many 1, 4, 7, 8 exist 

fn dowork(lines: io::Lines<io::BufReader<File>>) -> i64 {
    const SPLIT_STR: &str = "|";
    let mut counter: i64 = 0;

    for line in lines {
        if let Ok(instruction) = line {
            let mut inst = instruction.split(SPLIT_STR);
            inst.next();
            let with_whitespace = inst.next();
            let left_digits = with_whitespace.unwrap().split_whitespace();
            for dig in left_digits {
                match dig.len() {
                    2 => counter += 1,
                    3 => counter += 1,
                    4 => counter += 1,
                    7 => counter += 1,
                    _ => println!("nothin")
                }
            }
        }
    }
    counter
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    filename
} 