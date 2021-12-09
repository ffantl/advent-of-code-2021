use std::env;
use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    if let Ok(lines) = read_lines(filename) {
        let result = dowork(lines);
        println!("{}", result)
    }
}

// find median
// find average

fn find_min_and_max_fuel(min: i64, max: i64, input: HashMap<i64,i64>) -> ((i64, i64), (i64, i64)) {
    let mut min_index: i64 = 0;
    let mut max_index: i64 = 0;
    let mut min_result: i64 = 0;
    let mut max_result: i64 = 0;
    let mut set_initial_value: bool = true;

    for i in min..=max {
        let mut temp_result: i64 = 0;
        for (val, qt) in &input { // nth triangular number
            let x = (val - i).abs();
            temp_result += ((x * (x+1))/2) * qt;
        }
        if min_result > temp_result {
            min_result = temp_result;
            min_index = i;
        }
        if max_result < temp_result {
            max_result = temp_result;
            max_index = i;
        }
        if set_initial_value {
            min_index = i;
            max_index = i;
            min_result = temp_result;
            max_result = temp_result;
            set_initial_value = false;
            continue
        }
    }
    ((min_index, min_result), (max_index, max_result))
}

fn dowork(lines: io::Lines<io::BufReader<File>>) -> i64 {
    let mut initial: HashMap<i64,i64> = HashMap::new();
    const SPLIT_STR: &str = ",";
    let mut first_set: bool = true;
    let mut min: i64 = 0;
    let mut max: i64 = 0;

    for line in lines {
        if let Ok(instruction) = line {
            // parse instr
            // turn instr into x amount of coordinates
            // add them to hashmap and increase results by 1 if any line hits two.
            
            let crab_pos = instruction.split(SPLIT_STR);
            for crab in crab_pos {
                let index = crab.parse::<i64>().unwrap();
                let val = initial.entry(index).or_insert(0);
                *val += 1;

                if first_set {
                    min = index;
                    max = index;
                    first_set = false;
                    continue
                }

                if min > index {
                    min = index;
                }
                if max < index {
                    max = index;
                }
            }
        }
    }
    
    let (min_tuple, max_tuple)  = find_min_and_max_fuel(min, max, initial);
    println!("Min Fuel: {:?}\nMax Fuel:{:?}", min_tuple, max_tuple);

  
    min_tuple.0
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    filename
} 