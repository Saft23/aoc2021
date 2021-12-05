use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use core::ops::Sub;
fn move_line(vec: &mut Vec<Vec<i32>>, x1 : usize, y1 : usize, x2 : usize, y2 : usize){
    for x in x1..x2+1{
        for y in y1..y2+1{
            vec[y][x] = vec[y][x] + 1;
        }
    }
}

fn move_diagonal(vec: &mut Vec<Vec<i32>>, x1 : usize, y1 : usize, x2 : usize, y2 : usize){
    let abs_x = abs_difference(x1, x2);
    let change_x : i32;
    let change_y : i32;

    if x1 > x2{
        change_x = -1;
    }else{
        change_x = 1;
    }

    if y1 > y2{
        change_y = -1;
    }else{
        change_y = 1;
    }

    let mut i : i32 = 0;
    loop { //Negative for loops doesnt't work i RUST. Shitty language...
        let y_cord = y1 as i32+(i*change_y);
        let x_cord = x1 as i32+(i*change_x);
        vec[y_cord as usize][x_cord as usize] = vec[y_cord as usize][x_cord as usize] + 1;
        i = i + 1;
        if i-1 == abs_x as i32{
            break;
        }
    }
}

fn print_vec(vec: Vec<Vec<i32>>){
    for row in vec{
        for val in row{
            print!("{}",val);
        }
        println!("");
    }
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

fn calculate_atleast_2_crossings(vec: Vec<Vec<i32>>) -> i32{
    let mut sum = 0;
    for row in vec{
        for val in row{
            if val >= 2{
                sum = sum + 1;
            }
        }
    }
    return sum;
}
fn part1(){
    let lines = lines_from_file("../input.txt");
    let mut vec = vec![vec![0;1000];1000];
    for line in lines{
        let split_row = line.split(" ");
        let split_vec = split_row.collect::<Vec<&str>>();
        let start = split_vec[0];
        let stop = split_vec[2];

        let first_cord = start.split(",").collect::<Vec<&str>>();
        let second_cord = stop.split(",").collect::<Vec<&str>>();
        let start_x = first_cord[0].parse::<usize>().unwrap();
        let start_y = first_cord[1].parse::<usize>().unwrap();
        let stop_x = second_cord[0].parse::<usize>().unwrap();
        let stop_y = second_cord[1].parse::<usize>().unwrap();
        if start_x == stop_x || start_y == stop_y {
            if start_x > stop_x || start_y > stop_y{
                move_line(&mut vec, stop_x, stop_y, start_x, start_y);
            }else{
                move_line(&mut vec, start_x, start_y, stop_x, stop_y);
            }
        }
    }
    let part1_result = calculate_atleast_2_crossings(vec);
    println!("Part 1: {}",part1_result);

}

fn abs_difference<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}
fn part2(){
    let lines = lines_from_file("../input.txt");
    let mut vec = vec![vec![0;1000];1000];
    for line in lines{
        let split_row = line.split(" ");
        let split_vec = split_row.collect::<Vec<&str>>();
        let start = split_vec[0];
        let stop = split_vec[2];

        let first_cord = start.split(",").collect::<Vec<&str>>();
        let second_cord = stop.split(",").collect::<Vec<&str>>();
        let start_x = first_cord[0].parse::<usize>().unwrap();
        let start_y = first_cord[1].parse::<usize>().unwrap();
        let stop_x = second_cord[0].parse::<usize>().unwrap();
        let stop_y = second_cord[1].parse::<usize>().unwrap();
        if start_x == stop_x || start_y == stop_y { //Linear
            if start_x > stop_x || start_y > stop_y{
                move_line(&mut vec, stop_x, stop_y, start_x, start_y);
            }else{
                move_line(&mut vec, start_x, start_y, stop_x, stop_y);
            }
        }else if abs_difference(start_x, stop_x) == abs_difference(start_y , stop_y){ //Diagonal
                move_diagonal(&mut vec, stop_x, stop_y, start_x, start_y)
        }

    }
    let part2_result = calculate_atleast_2_crossings(vec);
    println!("Part 2: {}",part2_result);

}
fn main() {
    part1();
    part2();
}
