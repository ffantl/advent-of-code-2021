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

fn find_rate(index: usize, mut rates: Vec<String>, mostcommon: bool) -> String {
    let mut inner_ones: Vec<String> = Vec::new();
    let mut inner_zeroes: Vec<String> = Vec::new();
    let mut result: i32 = 0;

    if rates.len() == 1 {
        return rates.pop().unwrap();
    }
    
    for bits in rates {
        match bits.chars().nth(index).unwrap() {
            '0' => {result -= 1; inner_zeroes.push(bits)},
            '1' => {result += 1; inner_ones.push(bits)},
            _x => println!("wouldn't know")
        }
    }

    let newindex = index + 1;
    if result >= 0 {
        if mostcommon {
            return find_rate(newindex, inner_ones, mostcommon);
        } else {
            return find_rate(newindex, inner_zeroes, mostcommon);
        }
    } else {
        if mostcommon {
            return find_rate(newindex, inner_zeroes, mostcommon);
        } else {
            return find_rate(newindex, inner_ones, mostcommon);
        }
    }
}

fn dowork(lines: io::Lines<io::BufReader<File>>) -> i64 {
    let mut result: i64 = 0;
    let carbon_rate: i64;
    let carbon_rate_string: String;
    let oxygen_rate: i64;
    let oxygen_rate_string: String;


    let mut zeroes: Vec<String> = Vec::new();
    let mut ones: Vec<String> = Vec::new();


    for line in lines {
        if let Ok(bits) = line {
            match bits.chars().nth(0).unwrap() {
                '0' => {
                    result -= 1; 
                    zeroes.push(bits);
                },
                '1' => {
                    result += 1; 
                    ones.push(bits);
                },
                _x => println!("wouldn't know")
            }
        }
    }    
  
    if result >= 0 {
        // re-run for loop with 0s and go forward one
        carbon_rate_string = find_rate(1, zeroes, false);
        oxygen_rate_string = find_rate(1, ones, true);
    }  else {
        carbon_rate_string = find_rate(1, ones, false);
        oxygen_rate_string = find_rate(1, zeroes, true);
    }

    oxygen_rate = intfrombitstring(oxygen_rate_string);
    carbon_rate = intfrombitstring(carbon_rate_string);

    oxygen_rate * carbon_rate
}

fn intfrombitstring(bitstring: String) -> i64 {
    let mut ret: i64 = 0;
    let addfn = |i: usize| -> i64 {
        match i {
            0 => return 2048,
            1=> return 1024,
            2=> return 512,
            3=> return 256,
            4=> return 128,
            5=> return 64,
            6=> return 32,
            7=> return 16,
            8=> return 8,
            9=> return 4,
            10=> return 2,
            11=> return 1,
            _ => return 0
        }
    };

    for (i, bit) in bitstring.chars().enumerate() {
        match bit {
            '0' => ret += 0,
            '1' => ret += addfn(i),
            _x  => println!("again, no idea")
        }
    }

    return ret
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    filename
}