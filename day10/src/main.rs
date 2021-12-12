use std::{collections::HashSet, fs::File, io::{prelude::*, BufReader}, iter, path::Path};
use std::process;
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

fn part1(){
    let lines = lines_from_file("../input.txt");
    let mut soft = 0;
    let mut hard = 0;
    let mut curly = 0;
    let mut shark = 0;
    for line in lines{
        let mut next_close = vec![];
        for c in line.chars(){
            match c{
                '{' => next_close.push('}'),
                '<' => next_close.push('>'),
                '(' => next_close.push(')'),
                '[' => next_close.push(']'),
                _ => {
                    let g = next_close.pop().unwrap();
                    if c != g{
                        match c{
                            '}' => curly = curly + 1,
                            ')' => soft = soft + 1,
                            '>' => shark = shark + 1,
                            ']' => hard = hard + 1,
                            _ => println!("Fucked"),
                        }
                    }
                }
            };
        }
    }
    let result = soft * 3 + hard * 57 + curly * 1197 + shark * 25137;
    println!("Part 1: {}",result);
}

fn part2(){
    let lines = lines_from_file("../input.txt");
    let mut sum = vec![];
    for line in lines{
        let mut next_close = vec![];
        let mut corrupt = false;
        for c in line.chars(){
            match c{
                '{' => next_close.push('}'),
                '<' => next_close.push('>'),
                '(' => next_close.push(')'),
                '[' => next_close.push(']'),
                _ => {
                    let g = next_close.pop().unwrap();
                    if c != g{
                        corrupt = true;
                        break;
                    }
                }
            };
        }
        if !corrupt{
            next_close.reverse();
            let mut line_sum : i64 = 0;
            for c in next_close{
                line_sum = line_sum * 5;
                match c{
                    ')' => line_sum = line_sum + 1,
                    ']' => line_sum = line_sum + 2,
                    '}' => line_sum = line_sum + 3,
                    '>' => line_sum = line_sum + 4,
                    _ => println!("Fucked"),
                }
            }
            sum.push(line_sum);
        }
    }
    sum.sort();
    println!("Part 2: {}",sum[(sum.len()/2)]);
}
fn main() {
    part1();
    part2();
}
