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

fn get_coord_tuple_from_str(coord_str: &str) -> (i32,i32) {
    let mut split = coord_str.split(",");
    let coord: (i32, i32) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());
    coord
}

fn generate_coordinate_vec(start: (i32,i32), finish: (i32,i32)) -> Vec<(i32, i32)> {
    let mut coordinates_vec: Vec<(i32, i32)> = Vec::new();
    if start.0 != finish.0 && start.1 != finish.1 {
        return coordinates_vec
    }

    if start.0 - finish.0 == 0 { // x stays constant
        let dist = (finish.1 - start.1).abs();
        let dir = if finish.1 - start.1 >= 0 { 1 } else { -1 };
        for y in 0..=dist {
            coordinates_vec.push((start.0, start.1 + (y*dir)));
        }
    } else { // y stays constant
        let dist = (finish.0 - start.0).abs();
        let dir = if finish.0 - start.0 >= 0 { 1 } else { -1 };
        for x in 0..=dist { // inclusive of end
            coordinates_vec.push((start.0 + (x*dir), start.1));
        }
    }

    coordinates_vec
} 

fn dowork(lines: io::Lines<io::BufReader<File>>) -> i64 {
    let mut results: i64 = 0;
    let mut map: HashMap<(i32,i32), i32> = HashMap::new();
    const SPLIT_STR: &str = " -> ";
    
    for line in lines {
        if let Ok(instruction) = line {
            // parse instr
            // turn instr into x amount of coordinates
            // add them to hashmap and increase results by 1 if any line hits two.
            
            let mut coords = instruction.split(SPLIT_STR);
            let start = get_coord_tuple_from_str(coords.next().unwrap());
            let finish = get_coord_tuple_from_str(coords.next().unwrap());
            let coord_vec = generate_coordinate_vec(start, finish);
            println!("{:?}", coord_vec);
            for coord in coord_vec {
                let counter = map.entry(coord).or_insert(0);
                *counter += 1;
            }
        }
    }

    for (_key, value) in map.iter() {
        if *value >= 2 {
            results += 1
        }
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