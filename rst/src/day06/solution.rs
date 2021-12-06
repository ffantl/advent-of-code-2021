use std::env;
use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);
    let ticks: i32 = 256;

    if let Ok(lines) = read_lines(filename) {
        let result = dowork(lines, ticks);
        println!("{}", result)
    }
}

fn tick(input: HashMap<i64,i64>) -> HashMap<i64,i64> {
let mut output: HashMap<i64,i64> = HashMap::new();

for (k, v) in input.iter() {
        match k {
            0 => {
                {
                    let counter = output.entry(6).or_insert(0);
                    *counter += v;
                }
                {   
                    let babies = output.entry(8).or_insert(0);
                    *babies += v;
                }
            },
            1 => {
                let counter = output.entry(0).or_insert(0);
                *counter += v;
            },
            x => {
                let counter = output.entry(x-1).or_insert(0);
                *counter += v;
            }
        }
    }
    output
}

fn dowork(lines: io::Lines<io::BufReader<File>>, ticks: i32) -> i64 {
    let mut results: i64 = 0;
    let mut initial: HashMap<i64,i64> = HashMap::new();
    const SPLIT_STR: &str = ",";
    
    for line in lines {
        if let Ok(instruction) = line {
            // parse instr
            // turn instr into x amount of coordinates
            // add them to hashmap and increase results by 1 if any line hits two.
            
            let fish_days = instruction.split(SPLIT_STR);
            for fish_pos in fish_days {
                let val = initial.entry(fish_pos.parse::<i64>().unwrap()).or_insert(0);
                *val += 1;
            }
        }
    }
    
    let mut output: HashMap<i64,i64> = initial;
    for _i in 0..ticks {
        println!("Tick {}: {:?}", _i, output);
        let temp = tick(output);
        output = temp;
    }
    println!("After {}: {:?}", ticks-1, output);

    for (_key, value) in output.iter() {
        results += *value;
    }
  
    results
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    filename
} 