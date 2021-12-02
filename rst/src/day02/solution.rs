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

const FORWARD: &str = "forward";
const DOWN: &str = "down";
const UP: &str = "up";

fn dowork(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut horizontal_position: i32 = 0;
    let mut vertical_position: i32 = 0;
    let mut aim: i32 = 0;


    for line in lines {
        if let Ok(str_instr) = line {
            // parse the line to a string and i32
            let mut instr = str_instr.split_whitespace();
            let dir = instr.nth(0);
            let distance = instr.nth(0).unwrap().parse::<i32>().unwrap();
            match dir {
                Some(FORWARD) => {
                    horizontal_position += distance;
                    vertical_position += distance * aim;
                },
                Some(DOWN) => {
                    aim += distance;
                },
                Some(UP) => {
                    aim -= distance;
                },
                Some(x) => println!("not supported {}", x),
                None => println!("bad direction?")
            }
            
        }
    }
    
    println!("Horizontal: {} Vertical: {}", horizontal_position, vertical_position);
    horizontal_position * vertical_position
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    filename
}