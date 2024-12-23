use std::{env, fs};

fn main() {
    // file as argument
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // step 0: print the content of the file
    println!("~ File path: {file_path}");
    let content = fs::read_to_string(file_path)
        .expect("Error to read te file!");

    // step 1: sum of first and last number
    let mut sum: u32 = 0;
    for (_, line) in content.lines().enumerate() {
        let f_num = line
            .chars()
            .find(|c| c.is_numeric())
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0);
        let l_num = line
            .chars().rev()
            .find(|c| c.is_numeric())
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0);
        sum += 10 * f_num + l_num; 
    }
    println!("Total sum: {}", sum);
}
