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

fn dowork(lines: io::Lines<io::BufReader<File>>) -> i64 {
    let mut gamma_rate: i64 = 0 ; // Most common
    let mut epsilon_rate: i64 = 0 ; // Least common

    let mut result: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for line in lines {
        if let Ok(bits) = line {
            // parse the line to a string and i32
            
            for (i, bit) in bits.chars().enumerate() {
                match bit {
                    '0' => result[i] -= 1,
                    '1' => result[i] += 1,
                    _x => println!("wouldn't know")
                }
            }
        }
    }


    let mut gamma = |x: i64| gamma_rate += x;
    let mut epsi = |x: i64| epsilon_rate += x;
    
    if result[0] > 0 { gamma(2048)} else {epsi(2048)};
    if result[1] > 0 { gamma(1024)} else {epsi(1024)};
    if result[2] > 0 { gamma(512)} else {epsi(512)};
    if result[3] > 0 { gamma(256)} else {epsi(256)};
    if result[4] > 0 { gamma(128)} else {epsi(128)};
    if result[5] > 0 { gamma(64)} else {epsi(64)};
    if result[6] > 0 { gamma(32) } else {epsi(32)};
    if result[7] > 0 { gamma(16)} else {epsi(16)};
    if result[8] > 0 { gamma(8)} else { epsi(8)};
    if result[9] > 0 { gamma(4)} else { epsi(4)};
    if result[10] > 0 { gamma(2)} else { epsi(2)};
    if result[11] > 0 { gamma(1)} else { epsi(1)};

    gamma_rate * epsilon_rate
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    filename
}