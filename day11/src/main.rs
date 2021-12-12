use std::{collections::HashSet, default, fs::File, io::{prelude::*, BufReader}, iter, path::Path};
use std::process;
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

#[derive(Copy, Clone)]
struct Squid{
    val: i32,
    blinked: bool,
}

impl Default for Squid{
    fn default() -> Squid{
        Squid {val: 0, blinked: false}
    }
}

fn get_neighbors(x : i32, y : i32) -> Vec<(i32,i32)> {
    let mut res = vec![];
    for x in x-1..x+2{
        for y in y-1..y+2{
            if y != -1 && y != 10 && x != -1 && x != 10{
                res.push((x,y));
            }
        }
    }
    return res
}

pub trait Clone{
    fn clone(&self) -> Self;
}

fn part1(){
    let lines = lines_from_file("../input.txt");
    let mut sum = 0;
    let mut matrix = vec![vec![Squid { val: 0, blinked: false };10];10];
    for row in 0..lines.len(){
        for col in 0..lines[row].len(){
            let mut squid = Squid {
                val : (lines[row].as_bytes()[col]-48) as i32,
                blinked: false,
            };
            matrix[row][col] = squid;
        }
    }
    for _ in 0..100{
        //Increase all one step
        for y in 0..matrix.len(){
            for x in 0..matrix[y].len(){
                matrix[y][x].val = matrix[y][x].val + 1;
            }
        }

        let mut changes = 1;
        while changes != 0{
            changes = 0;
            //Neighbors
            for y in 0..matrix.len(){
                for x in 0..matrix[y].len(){
                    if matrix[x][y].val > 9 && !matrix[x][y].blinked{
                        matrix[x][y].blinked = true;
                        matrix[x][y].val = 0;
                        changes = changes + 1;
                        sum = sum + 1;
                        let neighbors = get_neighbors(x as i32, y as i32);
                        for n in neighbors {
                            if !matrix[n.0 as usize][n.1 as usize].blinked{
                            matrix[n.0 as usize][n.1 as usize].val = matrix[n.0 as usize][n.1 as usize].val + 1;
                            }
                        }
                    }
                }
            }


        }
        for y in 0..matrix.len(){
            for x in 0..matrix[y].len(){
                matrix[y][x].blinked = false;
            }
        }
    }
    println!("Part 1: {}",sum);
}
fn part2(){
    let lines = lines_from_file("../input.txt");
    let mut sum = 0;
    let mut matrix = vec![vec![Squid { val: 0, blinked: false };10];10];
    for row in 0..lines.len(){
        for col in 0..lines[row].len(){
            let squid = Squid {
                val : (lines[row].as_bytes()[col]-48) as i32,
                blinked: false,
            };
            matrix[row][col] = squid;
        }
    }
    for x in 0..10000{
        let mut flashes = 0;
        //Increase all one step
        for y in 0..matrix.len(){
            for x in 0..matrix[y].len(){
                matrix[y][x].val = matrix[y][x].val + 1;
            }
        }

        let mut changes = 1;
        while changes != 0{
            changes = 0;
            //Neighbors
            for y in 0..matrix.len(){
                for x in 0..matrix[y].len(){
                    if matrix[x][y].val > 9 && !matrix[x][y].blinked{
                        matrix[x][y].blinked = true;
                        flashes = flashes + 1;
                        matrix[x][y].val = 0;
                        changes = changes + 1;
                        sum = sum + 1;
                        let neighbors = get_neighbors(x as i32, y as i32);
                        for n in neighbors {
                            if !matrix[n.0 as usize][n.1 as usize].blinked{
                            matrix[n.0 as usize][n.1 as usize].val = matrix[n.0 as usize][n.1 as usize].val + 1;
                            }
                        }
                    }
                }
            }


        }
        for y in 0..matrix.len(){
            for x in 0..matrix[y].len(){
                matrix[y][x].blinked = false;
            }
        }

        if flashes == 100{
            println!("Part 2: {}",x+1);
            break;
        }
    }

}
fn main() {
    part1();
    part2();
}
