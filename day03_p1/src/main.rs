use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::char;

fn main() {
    let filename = "input.txt";

    let mut zeros = Vec::<u32>::new();
    let mut ones = Vec::<u32>::new();

    if let Ok(lines) = read_lines(filename) {
        
        let mut line_num = 0;
        lines.for_each(|line| {
            let chars: Vec<char> = line.unwrap().chars().collect();

            // initialize on first iter to length of the first line
            if line_num == 0 {
                let s = chars.len();
                zeros.resize(s, 0);
                ones.resize(s,0);
            }

            let mut i = 0;
            for c in chars {
                println!("line {}: {}", line_num, c);
                let x = c.to_digit(10).unwrap();
                if x == 0 {
                    
                    let b = zeros[i];
                    zeros[i] = b + 1;
                    
                } else {
                    
                    let b = ones[i];
                    ones[i] = b + 1;
                }
                i += 1;
            }

            line_num += 1;
        });

        println!("zeros: {:?}, ones {:?}", zeros, ones);

        let bit_length= zeros.len();
        println!("bit lenth: {}", bit_length);
        let mut g_bits = Vec::<char>::with_capacity(bit_length);
        g_bits.resize(bit_length, ' ');
        let mut e_bits = Vec::<char>::with_capacity(bit_length);
        e_bits.resize(bit_length, ' ');
        let mut i = 0;
        for z in zeros {

            let o = ones[i];

            if z > o {
                g_bits[i] = '1';
                e_bits[i] = '0';
            } else {
                g_bits[i] = '0';
                e_bits[i] = '1';
            }
            
            i += 1;
        }

        let gamma: String = g_bits.into_iter().collect();
        let epsilon: String = e_bits.into_iter().collect();
        println!("epsilon:\t{}\ngamma:\t\t{}", epsilon, gamma);

        let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
        let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();
        println!("epsilon:\t{}\ngamma:\t\t{}", epsilon_int, gamma_int);

        println!("final product: {}", gamma_int * epsilon_int);
    }

    
   
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}