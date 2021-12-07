use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input";

    if let Ok(lines) = read_lines(filename) {

        let mut h = 0;
        let mut d = 0;
        let mut a = 0;
        for line in lines {
            let line_string = line.unwrap().to_string();
            println!("{}",line_string);

            let split = line_string.split(" ");
            let v = split.collect::<Vec<&str>>();
            let dir = v[0];
            let num = v[1].parse::<i32>().unwrap();

            if dir == "forward" {
                h += num;
                d += a * num;
            } else if dir == "down" {
                a += num;
            } else if dir == "up" {
                a -= num;
            }
            
            println!("h:{}, d:{}", h, d)
        }

        let sum = h*d;
        println!("h: {}, d: {}, multiplied: {}", h, d, sum);
    }

    
   
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}