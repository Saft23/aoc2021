use std::{fs::File, io::{prelude::*, BufReader}, iter, path::Path};
use std::process;
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

fn get_str_difference(str1 : &str, str2 : &str) -> String{
    //Returns what str1 has that str2 doesn't

    let mut ret_string: String = "".to_owned();
    for c in str1.chars(){
        if !str2.contains(c){
            ret_string.push_str(&c.to_string());
        }
    }
    return ret_string;
}

fn get_real_number(val : &str, numbers : &Vec<&str>) -> i32{
    let mut cheats = vec!["";10];

    for val in numbers{
        match val.len(){
            2 => cheats[1] = val,
            3 => cheats[7] = val,
            4 => cheats[4] = val,
            7 => cheats[8] = val,
            _ => (),
        };

    }

    //Get the 0
    let one_four = get_str_difference(cheats[4], cheats[1]);
    for val in numbers{
        if val.len() == 6{
            if get_str_difference(&one_four, val).len() == 1{
                cheats[0] = val;
            }
        }
    }

    //Get the 6
    for val in numbers{
        if val.len() == 6{
            if get_str_difference(cheats[1], val).len() > 0{
                cheats[6] = val;
            }
        }
    }

    //Get the 9
    for val in numbers{
        if val.len() == 6{
            if val != &cheats[6] && val != &cheats[0]{
                cheats[9] = val;
            }
        }
    }

    //Get the 3, 5, 2
    for val in numbers{
        if val.len() == 5{
            if get_str_difference(cheats[1], val).len() == 0{
                cheats[3] = val;
            }
            if get_str_difference(cheats[6], val).len() == 1{
                cheats[5] = val;
            }
            if get_str_difference(cheats[6], val).len() == 2 && get_str_difference(cheats[1], val).len() > 0{
                cheats[2] = val;
            }

        }
    }

    for x in 0..cheats.len(){
        let mut n_num = 0;
        for c in val.chars(){
            n_num = n_num + 1;
            if !cheats[x].contains(c){
                break;
            }else if val.len() == cheats[x].len() && n_num == val.len(){
                return x as i32;
            }

        }
    }

    println!("Could not find value");
    return -1
}
fn part1(){
    let lines = lines_from_file("../input.txt");
    let mut encounters = 0;
    for line in lines{
        let split_vec = line.split("|").collect::<Vec<&str>>();
        let second_part = split_vec[1];

        let split_second_part = second_part.split(" ").collect::<Vec<&str>>();

        for val in split_second_part{
            if val.len() == 2 || val.len() == 4 || val.len() == 3 || val.len() == 7{
                encounters = encounters + 1;
            }
        }
    }
    println!("Part 1 {}",encounters);

}
fn part2(){
    let lines = lines_from_file("../input.txt");
    let mut sum = 0;
    for line in lines{
        let split_vec = line.split("|").collect::<Vec<&str>>();
        let first_part = split_vec[0].trim();
        let str_first = first_part.split(" ").collect::<Vec<&str>>();
        let second_part = split_vec[1];
        let split_second_part = second_part.split(" ").collect::<Vec<&str>>();
        let mut row_sum = 0;
        let base :i32 = 10;

        for i in 1..split_second_part.len(){
            row_sum = row_sum + get_real_number(split_second_part[i],&str_first) * base.pow((split_second_part.len() as u32) - i as u32 - 1);
        }
        sum = sum + row_sum;
    }
    println!("Part 2 {}",sum);

}
fn main() {
    part1();
    part2();
}
