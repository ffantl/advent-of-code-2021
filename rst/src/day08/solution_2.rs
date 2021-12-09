use std::env;
use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    if let Ok(lines) = read_lines(filename) {
        let result = dowork(lines);
        println!("{}", result)
    }
}

struct Possibilities {
    Pos: HashMap<char, HashSet<char>>
}

impl Possibilities {
    fn find_e(&self) -> Option<char> {
        for (c, set) in &self.Pos {
            if set.len() == 6 {
                return Some(*c);
            }
        }

        None
    }
}

const SEGMENTS: [char; 7] = ['a','b','c','d','e','f','g'];

fn decode_segment(all_segments: Vec<&str>) -> HashMap<String, char> {
    let mut translation_table: HashMap<String, char> = HashMap::new();
    let mut possibilities = Possibilities{ Pos: HashMap::new() };
    for Char in vec!['a','b','c','d','e','f','g'] {
        possibilities.Pos.insert(Char, SEGMENTS.to_vec().into_iter().collect());
    }

    let mut find_intersection = |character: char, poss: HashSet<char>| {
        let v = possibilities.Pos.entry(character).or_default();
        let inters: HashSet<char> = poss.intersection(v).copied().collect();
        *v = inters;
    };

    let seen_234 = (false, false, false);
    let mut six_lengths: HashSet<String> = HashSet::new();
    let mut five_lengths: HashSet<String> = HashSet::new();
    let mut acf_candidate: Vec<char> = Vec::new();
    let mut cf_candidate: Vec<char> = Vec::new();
    let mut bd_candidate: Vec<char> = Vec::new();

    for digit in all_segments {
        let mut chars: Vec<char> = digit.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));

        match chars.len() {
            2 => {
                // c,f
                find_intersection(chars[0], HashSet::from(['c', 'f']));
                find_intersection(chars[1], HashSet::from(['c', 'f']));
                cf_candidate = vec![chars[0], chars[1]];
                translation_table.insert(String::from_iter(chars), '1');
            },
            3 => {
                // a
                find_intersection(chars[0], HashSet::from(['a', 'c', 'f']));
                find_intersection(chars[1], HashSet::from(['a', 'c', 'f']));
                find_intersection(chars[2], HashSet::from(['a', 'c', 'f']));
                acf_candidate = vec![chars[0], chars[1], chars[2]];
                translation_table.insert(String::from_iter(chars), '7');
            }, 
            4 => {
                // b, d
                find_intersection(chars[0], HashSet::from(['b', 'c', 'd', 'f']));
                find_intersection(chars[1], HashSet::from(['b', 'c', 'd', 'f']));
                find_intersection(chars[2], HashSet::from(['b', 'c', 'd', 'f']));
                find_intersection(chars[3], HashSet::from(['b', 'c', 'd', 'f']));
                bd_candidate = vec![chars[0], chars[1], chars[2], chars[3]];
                translation_table.insert(String::from_iter(chars), '4');
            },
            7 => {
                translation_table.insert(String::from_iter(chars), '8');
            }, 
            6 => {
                six_lengths.insert(String::from_iter(chars));
            },
            5 => {
                five_lengths.insert(String::from_iter(chars));
            },
            _ => {}
        }
    }

    let find_outlier = |larger: &Vec<char>, smaller: &Vec<char>| -> Vec<char> {
        let mut leftover_char: Vec<char> = Vec::new();
        'outer: for value in larger {
            let mut found = false;
            for val in smaller {
                if value == val {
                    found = true;
                    continue 'outer;
                }
            }

            if !found {
                leftover_char.push(*value);
            }
        }
        leftover_char
    };
    
    let bd = find_outlier(&bd_candidate, &cf_candidate);
    let mut e: char = ' ';
    let mut b: char = ' ';
    
    // find_common 
    let six_one: HashSet<char> = six_lengths.iter().next().unwrap().chars().collect();
    let six_two: HashSet<char> = six_lengths.iter().next().unwrap().chars().collect();
    let six_three: HashSet<char> = six_lengths.iter().next().unwrap().chars().collect();
    let six_intersection: HashSet<char> = six_one.intersection(&six_two).copied().collect::<HashSet<char>>().intersection(&six_three).copied().collect();
    let c_value;
    let f_value;
    let b_value;
    let d_value;
    
    if let Some(_) = six_intersection.get(&bd[0]) {
        b_value = bd[0];
        d_value = bd[1];
    } else {
        b_value = bd[1];
        d_value = bd[0];
    }

    if let Some(_) = six_intersection.get(&cf_candidate[0]) {
        f_value = cf_candidate[0];
        c_value = cf_candidate[1];
    } else {
        f_value = cf_candidate[1];
        c_value = cf_candidate[0];
    }

    for s in six_lengths {
       // 6, 9, 0
       if let None = s.find(|x| x == d_value) {
           let zero_chars: Vec<char> = s.chars().collect();
        //    println!("{:?}", zero_chars);
           b = "abcdefg".replace(&zero_chars[..], "").chars().next().unwrap();
           translation_table.insert(String::from(s), '0');
       } else if let None = s.find(|x| x == c_value) {
           translation_table.insert(String::from(s), '6');
       } else {
           let nine_chars: Vec<char> = s.chars().collect();
           e = "abcdefg".replace(&nine_chars[..], "").chars().next().unwrap();
           translation_table.insert(String::from(s), '9');
       }
    }

    
    for s in five_lengths {
        if let Some(_) = s.find(|x| x == e) {
            translation_table.insert(String::from(s), '2');
        } else if let Some(_) = s.find(|x| x == b) {
            translation_table.insert(String::from(s), '5');
        } else {
            translation_table.insert(String::from(s), '3');
        }
    }
    translation_table    
}

fn dowork(lines: io::Lines<io::BufReader<File>>) -> i64 {
    const SPLIT_STR: &str = "|";
    let mut counter: i64 = 0;

    for line in lines {
        if let Ok(instruction) = line {
            let mut inst = instruction.split(SPLIT_STR);
            let with_whitespace = inst.next();
            let left_digits = with_whitespace.unwrap().split_whitespace();
            let translation_map = decode_segment(left_digits.collect());
            println!("Map {:?}", translation_map);

            let right_with_whitespace = inst.next();
            let right_digits = right_with_whitespace.unwrap().split_whitespace();
            let mut number: Vec<char> = Vec::new();
            for digit in right_digits {
                let mut chars: Vec<char> = digit.chars().collect();
                chars.sort_by(|a, b| a.cmp(b));
                let sorted_chars = chars;
                let needed = *translation_map.get(&String::from_iter(sorted_chars)).unwrap();
                println!("Digit: {} Value: {}", digit, needed);
                number.push(needed);
            }
            let value = String::from_iter(number);
            println!("{} : {:?}", value, right_with_whitespace);
            counter += value.parse::<i64>().unwrap();
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