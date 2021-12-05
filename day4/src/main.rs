use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct Board {
    grid: Vec<Vec<i32>>,
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

impl Board {
    fn has_won(&self, numbers: &Vec<i32>) -> bool{
        let mut rows_in_row = 0;
        let mut col1 = 0;
        let mut col2 = 0;
        let mut col3 = 0;
        let mut col4 = 0;
        let mut col5 = 0;

        println!("Raffle is: ");
        for val in numbers{
            print!("{}, ",val);
        }
        println!("End");

        for row in &self.grid{
            for i in 0..row.len(){
                if numbers.contains(&row[i]){
                    match i{
                        0 => col1 = col1 + 1,
                        1 => col2 = col2 + 1,
                        2 => col3 = col3 + 1,
                        3 => col4 = col4 + 1,
                        4 => col5 = col5 + 1,
                        _ => println!("What is this"),
                    }
                    rows_in_row = rows_in_row + 1;
                    println!("checking: {} and row is: {}",&row[i], rows_in_row);
                    if rows_in_row == 5 || col1 == 5 || col2 == 5 || col3 == 5 || col4 == 5 || col5 == 5{
                        return true;
                    }
                }
            }
            rows_in_row = 0;
        }
        return false;
    }
    fn calculate_victory(&self, numbers: &Vec<i32>) -> i32{
        let mut calling_sum = 0;
        for row in &self.grid{
            for val in row{
                if !numbers.contains(val){
                    println!("Adding: {}", val);
                    calling_sum = calling_sum + val;
                }
            }
        }
        println!("Final num is: {}",numbers[numbers.len()-1]);
        return calling_sum * numbers[numbers.len()-1];
    }
}
fn part1() {
    let lines = lines_from_file("../input.txt");
    let mut boards = vec![];
    let mut raffle_int = vec![];
    let mut board = Board {
        grid: vec![vec![]],
    };
    for i in 0..lines.len(){
        let mut v = vec![];
        if i == 0{
            let raffle = lines[i].split(",");
            for val in raffle{
                if val != "" {
                    raffle_int.push(val.parse::<i32>().unwrap());
                }
            }
        }else if lines[i] == "" && i != 1{
            println!("New board");
            boards.push(board);
            board = Board {
                grid: vec![vec![]],
            };
        }else{
            let row = lines[i].split(" ");
            for val in row{
                if val == ""{
                    continue;
                }
                println!("{}",val);
                let res = val.parse::<i32>().unwrap();
                v.push(val.parse::<i32>().unwrap());
            }
            if v.len() == 5{
                board.grid.push(v);
            }
        }
    }
    boards.push(board);
   // println!("{}",boards.len());
   // for board in boards{
   //     println!("{}",board.grid.len());
   //     for row in board.grid{
   //         for val in row{
   //             print!("{} ",val);
   //         }
   //         println!("");
   //     }
   // }
    let mut callout = vec![];
    for raf in raffle_int{
        callout.push(raf);
        for board in &boards{
            if board.has_won(&callout){
                println!("Board has won with score: {}",board.calculate_victory(&callout));

                for row in &board.grid{
                    for val in row{
                        print!("{} ",val);
                    }
                    println!("");
                }
                return
            }
        }
    }
}
fn main() {
    part1();
}
