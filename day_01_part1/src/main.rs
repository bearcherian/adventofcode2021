use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input";

    if let Ok(lines) = read_lines(filename) {

        let mut larger_measurements = 0;
        let mut previous = 0;
        let mut iteration = 0;
        for line in lines {
            let line_string = line.unwrap().to_string();
            let current = line_string.parse::<i32>().unwrap();

            if iteration > 0 &&  current > previous {
                larger_measurements += 1;
            }
            previous = current;
            iteration += 1;
        }

        println!("{}", larger_measurements);
    }

    
   
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}