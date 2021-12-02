use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};


struct Submarine {
    horizontal_pos: i32,
    depth: i32,
    aim: i32,
}
impl Submarine {
    fn down(&mut self, change:i32) {
        self.depth = self.depth + change;
    }
    fn up(&mut self, change:i32) {
        self.depth = self.depth - change;
    }
    fn forward(&mut self, change:i32) {
        self.horizontal_pos = self.horizontal_pos + change;
    }
    fn down2(&mut self, change:i32) {
        self.aim = self.aim + change;
    }
    fn up2(&mut self, change:i32) {
        self.aim = self.aim - change;
    }
    fn forward2(&mut self, change:i32) {
        self.horizontal_pos = self.horizontal_pos + change;
        self.depth = self.depth + (self.aim * change);
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()

}

fn part1(){
    let mut sub = Submarine {
        horizontal_pos: 0,
        depth: 0,
        aim: 0,
    };
    let lines = lines_from_file("input.txt");
    for line in lines{
        let split = line.split(" ");
        let vec = split.collect::<Vec<&str>>();
        let function = vec[0].to_string();
        let value = vec[1].parse::<i32>().unwrap();
        match function.as_ref() {
            "forward" => sub.forward(value),
            "down" => sub.down(value),
            "up" => sub.up(value),
            _ => println!("Fucked up"),
        }
    }

    println!("{}",sub.depth * sub.horizontal_pos);
}

fn part2(){
    let mut sub = Submarine {
        horizontal_pos: 0,
        depth: 0,
        aim: 0,
    };
    let lines = lines_from_file("input.txt");
    for line in lines{
        let split = line.split(" ");
        let vec = split.collect::<Vec<&str>>();
        let function = vec[0].to_string();
        let value = vec[1].parse::<i32>().unwrap();
        match function.as_ref() {
            "forward" => sub.forward2(value),
            "down" => sub.down2(value),
            "up" => sub.up2(value),
            _ => println!("Fucked up"),
        }
    }

    println!("{}",sub.depth * sub.horizontal_pos);

}
fn main() {
    part1();
    part2();
}
