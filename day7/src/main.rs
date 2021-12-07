use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use core::ops::Sub;
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

fn part1(){
    let lines = lines_from_file("../input.txt");
    let crabs = &lines[0];
    let vec_crabs = crabs.split(",").collect::<Vec<&str>>();
    let mut vec_crabs_int = vec![];
    for crab in vec_crabs{
        vec_crabs_int.push(crab.parse::<i32>().unwrap());
    }

    let min_value : i32 = *vec_crabs_int.iter().min().unwrap();
    let max_value : i32 = *vec_crabs_int.iter().max().unwrap();

    let mut final_task = 9999999;
    let mut best_task = 0;
    for x in min_value..max_value{
        for crab in &vec_crabs_int{
            best_task = best_task +  (crab - x).abs();
        }

        if best_task < final_task{
            final_task = best_task;
        }
        best_task = 0;
    }

    println!("Part 1: {}",final_task);
}


fn part2(){
    let lines = lines_from_file("../input.txt");
    let crabs = &lines[0];
    let vec_crabs = crabs.split(",").collect::<Vec<&str>>();
    let mut vec_crabs_int = vec![];
    for crab in vec_crabs{
        vec_crabs_int.push(crab.parse::<i32>().unwrap());
    }

    let min_value : i32 = *vec_crabs_int.iter().min().unwrap();
    let max_value : i32 = *vec_crabs_int.iter().max().unwrap();

    let mut final_task : i64 = 9999999999;
    let mut best_task : i64 = 0;
    for x in min_value..max_value{
        for crab in &vec_crabs_int{
            for i in 1..(crab - x).abs()+1{
                best_task = best_task + i as i64;
                if best_task > final_task{
                    break;
                }
            }
        }
        if best_task < final_task{
            final_task = best_task;
        }
        best_task = 0;
    }

    println!("Part 2: {}",final_task);
}
fn main() {
    part1();
    part2();
}
