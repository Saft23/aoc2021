use std::{collections::HashSet, fs::File, io::{prelude::*, BufReader}, iter, path::Path};
use std::process;
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

fn get_lowests_points(grid : &Vec<Vec<i32>>) -> Vec<(usize,usize)>{
    let mut lowest_points = vec![];
    for y in 0..grid.len(){
        for x in 0..grid[y].len(){
            let mut up = 9;
            let mut down = 9;
            let mut left = 9;
            let mut right = 9;

            let curr = grid[y][x];
            if y >= 1{
                up = grid[y-1][x];
            }
            if y+1 <= grid.len()-1{
                down = grid[y+1][x];
            }
            if x >= 1{
                left = grid[y][x-1];
            }
            if x+1 <= grid[y].len()-1{
                right = grid[y][x+1];
            }

            if curr < up && curr < down && curr < left && curr < right{
                lowest_points.push((y,x));
            }
        }
    }
    return lowest_points;
}

fn get_baisin_size(point : (usize, usize), grid: &Vec<Vec<i32>>) -> usize{

    let mut points = vec![];
    points.push(point);
    loop{
        let mut new_points = vec![];
        for point in &points{
            //Up
            if point.0 >= 1{
                let new_y = point.0-1;
                let new_x = point.1;
                if !points.contains(&(new_y,new_x)) && !new_points.contains(&(new_y,new_x)){
                    if grid[new_y][new_x] != 9{
                        new_points.push((new_y,new_x));
                    }
                }
            }
            //Down
            if point.0+1 <= grid.len()-1{
                let new_y = point.0+1;
                let new_x = point.1;
                if !points.contains(&(new_y,new_x)) && !new_points.contains(&(new_y,new_x)){
                    if grid[new_y][new_x] != 9{
                        new_points.push((new_y,new_x));
                    }
                }
            }
            //Left
            if point.1 >= 1{
                let new_y = point.0;
                let new_x = point.1-1;
                if !points.contains(&(new_y,new_x)) && !new_points.contains(&(new_y,new_x)){
                    if grid[new_y][new_x] != 9{
                        new_points.push((new_y,new_x));

                    }
                }
            }
            //Right
            if point.1+1 <= grid[point.0].len()-1{
                let new_y = point.0;
                let new_x = point.1+1;
                if !points.contains(&(new_y,new_x)) && !new_points.contains(&(new_y,new_x)){
                    if grid[new_y][new_x] != 9{
                        new_points.push((new_y,new_x));
                    }
                }
            }
        }
        if new_points.len() == 0{
            return points.len();
        }else{
            for val in new_points{
                points.push(val);
            }

        }
    }
}
fn part1(){
    let lines = lines_from_file("../input.txt");
    let rows = lines.len();
    let cols = lines[0].len();
    let mut grid = vec![vec![0;cols];rows];
    let mut lineno = 0;
    for line in lines{
        let mut cno = 0;
        for c in line.chars(){
            grid[lineno][cno] = c as i32 - 48;
            cno = cno + 1;
        }
        lineno = lineno + 1;
    }

    let part1_res = get_lowests_points(&grid);
    let mut sum = 0;
    for cord in part1_res{
        sum = sum + grid[cord.0][cord.1] + 1;
    }
    println!("Part 1: {}",sum);
}
fn part2(){
    let lines = lines_from_file("../input.txt");
    let rows = lines.len();
    let cols = lines[0].len();
    let mut grid = vec![vec![0;cols];rows];
    let mut lineno = 0;
    for line in lines{
        let mut cno = 0;
        for c in line.chars(){
            grid[lineno][cno] = c as i32 - 48;
            cno = cno + 1;
        }
        lineno = lineno + 1;
    }

    let lowest_points = get_lowests_points(&grid);

    let mut basins = vec![];
    for point in lowest_points{
        basins.push(get_baisin_size(point, &grid));
    }

    basins.sort();
    basins.reverse();
    let sum = basins[0] * basins[1] * basins[2];
    println!("Part 2: {}",sum);

}
fn main() {
    part1();
    part2();
}
