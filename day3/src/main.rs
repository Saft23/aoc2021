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
    let lines = lines_from_file("../input.txt");
    let mut array: [i32; 12] = [0; 12];
    for line in lines{
        for i in 0..line.len() {
            let current_char = line.chars().nth(i).unwrap();
            match current_char {
                '1' => array[i] = array[i] + 1,
                '0' => (),
                _ => println!("Fucked up"),
            }
        }
    }

    let mut gammarate = "".to_string();
    let mut epsilonrate = "".to_string();
    for count in array {
        if count > 500{
            gammarate.push_str("1");
            epsilonrate.push_str("0");
        }else if count < 500{
            gammarate.push_str("0");
            epsilonrate.push_str("1");
        }else {
            println!("fucked");
        }
    }

    let gammaint = isize::from_str_radix(&gammarate, 2).unwrap();
    let epsilonint = isize::from_str_radix(&epsilonrate, 2).unwrap();
    println!("Part 1: {}",gammaint*epsilonint);
}

fn part2(){
    let mut lines = lines_from_file("../input.txt");

    //Oxygen rating
    let mut oxygen_rating = 0;
    for i in 0..12{
        let to_save_char = check_most_used_bit(&lines, i);
        let mut new_vec = Vec::new();
        for line in lines{
            if line.chars().nth(i).unwrap() == to_save_char{
                new_vec.push(line);
            }
        }
        if new_vec.len() == 1{
            oxygen_rating = isize::from_str_radix(&new_vec[0], 2).unwrap();
        }
        lines = new_vec;
    }

    let mut lines = lines_from_file("../input.txt");

    //CO2 rating
    let mut co2_rating = 0;
    for i in 0..12{
        let mut to_save_char = check_most_used_bit(&lines, i);
        if to_save_char == '0'{
           to_save_char = '1';
        }else{
           to_save_char = '0';
        }
        let mut new_vec = Vec::new();
        for line in lines{
            if line.chars().nth(i).unwrap() == to_save_char{
                new_vec.push(line);
            }
        }
        if new_vec.len() == 1{
            co2_rating = isize::from_str_radix(&new_vec[0], 2).unwrap();
        }
        lines = new_vec;
    }
    println!("Part 2: {}",oxygen_rating*co2_rating);
}

fn check_most_used_bit(list : &Vec<String>, index: usize) -> char{
    let mut ones = 0;
    let mut zeroes = 0;
    for line in list{
        let current_char = line.chars().nth(index).unwrap();
        match current_char {
            '1' => ones = ones + 1,
            '0' => zeroes = zeroes + 1,
            _ => println!("Fucked up"),
        }
    }
    if ones >= zeroes {
        return '1';
    }
    return '0';

}

fn main() {
    part1();
    part2();
}
