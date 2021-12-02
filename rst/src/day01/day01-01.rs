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
    let mut totalCount: i32 = 0;
    let mut lastDepth: i32 = 0;

    for line in lines {
        if let Ok(str_depth) = line {
            let depth = str_depth.parse::<i32>().unwrap();
            if lastDepth == 0 {
            } else if lastDepth < depth {
                totalCount += 1;
            }
            lastDepth = depth
        }
    }

    totalCount
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    filename
}