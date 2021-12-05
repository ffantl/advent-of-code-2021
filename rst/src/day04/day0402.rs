use std::fs::File;
use std::path::Path;
use std::env;
use std::io::BufRead;
use std::io;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    if let Ok(lines) = read_lines(filename) {
        let result = dowork(lines);
        println!("{}", result)
    }
}


// if a number is called 10
// find
#[derive(Clone,Debug)]
struct BingoBoard {
    // col (how many values to the right is the col)
    // row (how many values down is the row)

    // y pos, x pos
    weird_champ: Vec<Vec<i32>>,
    col_lines: Vec<i32>,
    row_lines: Vec<i32>,
    index_maps: HashMap<String, (usize, usize)>
}



// Bingo Board
// 1 2 3 4 5
// 6 7 8 9 10
// 1 1 1 1 1
// 1 1 1 1 1
// 1 1 1 1 1



fn dowork(lines: io::Lines<io::BufReader<File>>) -> i64 {
    let num_withdrawals: Vec<&str>;
    let bingo_boards: &mut Vec<BingoBoard> = &mut Vec::new();
    let mut bingo_boards_index: usize = 0;
    let mut height: usize = 0;
    let splitchar = ",";
    let sum = |x: Vec<i32>| -> i32 {let mut result = 0; for num in x {if num == 0 {result += 999} else {result += num}}; result};
    // Start at -1 to increase the first time we see a bingo board, for accurate index
    // -1 -> 0 (First Board is placed)


    let mut iter = lines.enumerate();
    let (_, first_value) = iter.next().unwrap();
    let clone = first_value.unwrap().clone();
    num_withdrawals = clone.split(splitchar).collect();

    
    

    // SETUP
    for (_, line) in iter {
        if let Ok(bingo_line) = line {
            if bingo_line.is_empty() {
                if bingo_boards.len() != 0 {
                    bingo_boards_index += 1;
                }

                
                bingo_boards.push(BingoBoard{
                    weird_champ: (0..5).map(|_| vec![0; 5]).collect(),
                    col_lines: vec![0; 5],
                    row_lines: vec![0; 5],
                    index_maps: HashMap::new()
                });
                height = 0;
                continue
            }
            
            let current = bingo_boards.get_mut(bingo_boards_index).unwrap();
            let numbers = bingo_line.split_whitespace();
            let numline = numbers.clone().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

            current.row_lines[height] = sum(numline);

            for (length, number) in numbers.enumerate() {
                current.index_maps.insert(number.to_string(), (height, length));
                let mut value = number.parse::<i32>().unwrap();
                let val = current.col_lines.get_mut(length).unwrap();
                if value == 0 {
                    value = 999;
                }
                *val = *val + value;
                current.weird_champ[height][length] = value;
            }
            height += 1;
        }
    }

    // PLAY LOOP
    let mut winner_index_option: Option<usize> = None;
    let mut final_number: i32 = 0; // haha this should be an option
    
    // Operations to apply to each board
    let mut col_changes: Vec<Vec<i32>> = (0..bingo_boards.len()).map(|_| vec![0; 5]).collect();
    let mut row_changes: Vec<Vec<i32>> = (0..bingo_boards.len()).map(|_| vec![0; 5]).collect();
    let mut remove_indices: Vec<Vec<(usize, usize)>> = (0..bingo_boards.len()).map(|_| Vec::new()).collect();
    let mut remaining_indices = bingo_boards.len();
    let mut skip_index: HashMap<usize, ()> = HashMap::new();
    'outer: for number in num_withdrawals {
        let num: i32;
        if number.parse::<i32>().unwrap() == 0 {
            num = 999
        } else {
            num = number.parse::<i32>().unwrap();
        }
        println!("Col Changes: {:?}", col_changes);
        println!("Row Changes: {:?}\n", row_changes);
        
        'inner: for (i, board) in bingo_boards.iter().enumerate() {
            let found: (usize, usize);
            if skip_index.contains_key(&i) {
                continue 'inner;
            }

            match board.index_maps.get(number) {
                Some(indeces) => {found = *indeces; remove_indices[i].push(found)}
                None => {continue 'inner;}
            }


            col_changes[i][found.1] += num;
            row_changes[i][found.0] += num;
            
            let col_total: i32 = *board.col_lines.get(found.1).unwrap();
            let row_total: i32 = *board.row_lines.get(found.0).unwrap();
            if col_total - col_changes[i][found.1] == 0 {
                if remaining_indices == 1 {
                    winner_index_option = Some(i);
                    final_number = num;
                    println!("Loser Board ({}) Column Total: {}", i, col_total);
                    break 'outer;
                } else {
                    skip_index.insert(i, ());
                    remaining_indices -= 1;
                }
            } else if row_total - row_changes[i][found.0] == 0 {
                if remaining_indices == 1 {
                    winner_index_option = Some(i);
                    final_number = num;
                    println!("Loser Board ({}) Row Total: {}", i, row_total);
                    break 'outer;
                } else {
                    skip_index.insert(i, ());
                    remaining_indices -= 1;
                }
            }
       }
    }


    match winner_index_option {
        Some(winner_index) => {
            if final_number == 999 {
                final_number = 0;
            }

            println!("Winner Index: {:?}, Final Number {}", winner_index, final_number);
            calculate_from_weirdchamp(&bingo_boards[winner_index].weird_champ, &remove_indices[winner_index], final_number);
            let sum_of_all = sum(bingo_boards[winner_index].row_lines.clone()) - sum(col_changes[winner_index].clone());
            return (sum_of_all * final_number) as i64
        }
        None => {
            println!("None found. Why though");
            return 0
        }
    }
}

fn calculate_from_weirdchamp(weird_champ: &Vec<Vec<i32>>, changes: &Vec<(usize, usize)>, last_num: i32) -> i64 {
    let mut weird_copy: Vec<Vec<i32>> = (0..5).map(|_| vec![0; 5]).collect();
    let mut index: usize = 0;
    for ind in weird_champ {
        weird_copy[index] = ind.clone();
        index += 1;
    }
    
    for change in changes {
        weird_copy[change.0][change.1] = 0;
    }
    
    let mut result = 0;
    for champ in weird_copy {
        for champchamp in champ {
            if champchamp == 999 {
                result += 0
            } else {
                result += champchamp;
            }
        }
    }

    result = result*last_num;
    println!("Weirdchamp Result: {}", result);
    println!("Weirdchamp: {:?}", weird_champ);
    result as i64
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    filename
}