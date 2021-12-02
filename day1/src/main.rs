use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part1(){
    let mut number_one = 0;
    let mut number_two;
    let mut increased = -1;
    let lines = lines_from_file("input.txt");
        for line in lines {
                number_two = line.parse::<i32>().unwrap();
                if number_two > number_one{
                    increased = increased + 1;
                }
                number_one = number_two;
            }
    println!("{}", increased);
}

fn part2(){
    let lines = lines_from_file("input.txt");
    let mut first = 0;
    let mut second = 0;
    let mut third;
    let mut prev = 0;
    let mut increased = -3;
    for line in lines{
        let current_val = line.parse::<i32>().unwrap();
        third = second;
        second = first;
        first = current_val;
        if first + second + third > prev {
            increased = increased + 1;
        }
        prev = first + second + third;
    }
    println!("{}",increased)
}

fn main() {
    part1();
    part2();
}

