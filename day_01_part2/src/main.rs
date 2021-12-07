use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    let filename = "input";

    if let Ok(lines) = read_lines(filename) {

        // setup 3 windows and a vector of ordered windows
        let mut window_1 = Vec::<i32>::new();
        let mut window_2 = Vec::<i32>::new();
        let mut window_3 = Vec::<i32>::new();
        let mut windows = Vec::<Vec::<i32>>::new();
        
        let mut i = 0;
        // using a for_each because we have ownership problems with the vectors in a for loop
        lines.for_each(|line| {
            let s = line.unwrap().to_string();
            let m = s.parse::<i32>().unwrap();

            // stagger when we populate the windows
            if i == 0 {
                window_1.push(m);
            } else if i == 1 {
                window_1.push(m);
                window_2.push(m);
            } else {
                window_1.push(m);
                window_2.push(m);
                window_3.push(m);
            }

            
            // check each window, and if we have 3, save it and reset it
            if window_1.len() == 3 {
                windows.push(window_1.to_vec());
                window_1.clear();
            }

            if window_2.len() == 3 {
                windows.push(window_2.to_vec());
                window_2.clear();
            }

            if window_3.len() == 3 {
                windows.push(window_3.to_vec());
                window_3.clear();
            }
            

            i += 1;
        });

        let mut larger_measurements = 0;
        let mut previous = 0;
        let mut i = 0;
        // iterate over the collected windows to check if which sums are increasing
        for w in &windows {

            // add up the values of the window
            let mut sum = 0;
            for x in w {
                sum += x;
            }

            if i > 0 && sum > previous {
                larger_measurements += 1;
            }

            previous = sum;
            i += 1;

        }
    
        
        println!("{}", larger_measurements);
    }

    
   
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}