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
    println!("Input: {:?} {:?}", start, finish);

    let find_dir = |start: i32, finish: i32| -> i32 {if finish - start >= 0 {1} else {-1}};
    
    let mut coordinates_vec: Vec<(i32, i32)> = Vec::new();
    let dist = ((finish.0 - start.0).abs(), (finish.1 - start.1).abs());
    let dir = ((find_dir(start.0, finish.0)), find_dir(start.1, finish.1));

    if start.0 - finish.0 == 0 { // x stays constant
        for y in 0..=dist.1 {
            coordinates_vec.push((start.0, start.1 + (y*dir.1)));
        }
    } else if start.1 - finish.1 == 0 { // y stays constant
        for x in 0..=dist.0 { // inclusive of end
            coordinates_vec.push((start.0 + (x*dir.0), start.1));
        }
    } else { // ANGLES
        // must have a 45 degree angle.
        let slope = (dist.1*dir.1) as f64/(dist.0*dir.0) as f64;
        let b = ((start.0 as f64 * slope) - start.1 as f64) * -1 as f64;
        if slope.abs() != 0.5 && slope.abs() != 1.0 {
            println!("slope (rejected): {}", slope);
            return coordinates_vec
        }
        println!("function y = {}x + {}", slope, b);

        for x in 0..=dist.0 {
            coordinates_vec.push((start.0 + (x * dir.0), ((slope * (start.0 + (x*dir.0)) as f64) + b) as i32));
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
            println!("Output: {:?}\n", coord_vec);
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