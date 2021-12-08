use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashMap,
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
    let fishes = &lines[0];
    let vec_fishes = fishes.split(",").collect::<Vec<&str>>();
    let mut vec_fishes_int = vec![];
    for fish in vec_fishes{
        vec_fishes_int.push(fish.parse::<i32>().unwrap());
    }

    for x in 0..80{
        let mut new_vec_fishes_int = vec![];
        for i in 0..vec_fishes_int.len(){
            match vec_fishes_int[i]{
                0 => {
                    new_vec_fishes_int.push(8);
                    new_vec_fishes_int.push(6)
                }
                _ => new_vec_fishes_int.push(vec_fishes_int[i]-1),
            }
        }
        vec_fishes_int = new_vec_fishes_int;
    }

    println!("{}",vec_fishes_int.len());
}
fn part2(){
    let lines = lines_from_file("../input.txt");
    let fishes = &lines[0];
    let vec_fishes = fishes.split(",").collect::<Vec<&str>>();
    let mut vec_fishes_int = vec![];
    for fish in vec_fishes{
        vec_fishes_int.push(fish.parse::<i32>().unwrap());
    }

    let mut fishes = vec![0;9];

    for i in vec_fishes_int {
        fishes[i as usize] = fishes[i as usize] + 1;
    }

    for x in 0..256{
        let mut new_fishes = vec![0;9];
        new_fishes[0] = fishes[1];
        new_fishes[1] = fishes[2];
        new_fishes[2] = fishes[3];
        new_fishes[3] = fishes[4];
        new_fishes[4] = fishes[5];
        new_fishes[5] = fishes[6];
        new_fishes[6] = fishes[7] + fishes[0];
        new_fishes[7] = fishes[8];
        new_fishes[8] = fishes[0];

        fishes = new_fishes;
    }

    let res : i64 = fishes.iter().sum();
    println!("Part 2 {}",res);
}

fn main() {
    part1();
    part2();
}
