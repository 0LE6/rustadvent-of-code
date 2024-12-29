use std::env;
use std::fs::File;
use std::io::{self, BufRead};

// https://adventofcode.com/2023/day/1

fn main() -> io::Result<()> {
    // file as argument
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // step 0: print the content of the file
    println!("~ File path: {file_path}");
    // let content = fs::read_to_string(file_path)
        // .expect("Error to read te file!");
    let file = File::open("doc.txt")?;
    let content = io::BufReader::new(file);

    // step 1: sum of first and last number
    let mut sum: u32 = 0;
    // for (_, line) in content.lines().enumerate() {
    for line in content.lines() {
        let line = line?;
        let f_num = line
            .chars()
            .find(|c| c.is_numeric())
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0);
        println!("f_num ::: {}", f_num);
        let l_num = line
            .chars().rev()
            .find(|c| c.is_numeric())
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0);
        println!("l_num ::: {}", l_num);
        sum += 10 * f_num + l_num;
        // sum += comb_num;
        println!("sum ::: {}", sum); 
    }
    println!("Total sum: {}", sum);
    
    Ok(())
}
