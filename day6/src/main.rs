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
        println!("{}",x);
        vec_fishes_int = new_vec_fishes_int;
    }

    println!("{}",vec_fishes_int.len());
}
fn part2(){
    let lines = lines_from_file("../input2.txt");
    let fishes = &lines[0];
    let vec_fishes = fishes.split(",").collect::<Vec<&str>>();
    let mut vec_fishes_int = vec![];
    for fish in vec_fishes{
        vec_fishes_int.push(fish.parse::<i32>().unwrap());
    }

    for x in 0..256{
    }

    println!("{}",vec_fishes_int.len());

}
fn main() {
    part1();
    part2();
}
