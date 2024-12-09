use std::{env, mem};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use regex::Regex;

fn main() {
    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let memory = read_memory_data(file_path).expect("Failed to read and parse file");

    println!("Memory rows found: {:?}", memory.len());
    
    println!("==================== Part 1 ====================");
    let mul_regex = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();

    let mut sum: i32 = 0;

    for s in memory.iter() {
        for mul_capture in mul_regex.captures_iter(s.as_str()) {
            let first: i32 = mul_capture.name("first").unwrap().as_str().parse().unwrap();
            let second: i32 = mul_capture.name("second").unwrap().as_str().parse().unwrap();
            sum += first * second;
        }
    }
    println!("Sum: {}", sum);

    println!("==================== Part 2 ====================");
    let mul_regex_with_dos = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut do_mul = true;
    sum = 0;
    for s in memory.iter() {
        for command_capture in mul_regex_with_dos.captures_iter(s.as_str()) {
            if let Some(mul_capture) = command_capture.name("first") {
                if do_mul {
                    let first: i32 = mul_capture.as_str().parse().unwrap();
                    let second: i32 = command_capture.name("second").unwrap().as_str().parse().unwrap();
                    sum += first * second;
                }
            } else if command_capture.get(0).unwrap().as_str() == "do()" {
                do_mul = true;
            } else if command_capture.get(0).unwrap().as_str() == "don't()" {
                do_mul = false;
            }
        }
    }

    println!("Sum: {}", sum);

}

fn read_memory_data(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut memory = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            memory.push(line);
        }
    }
    Ok(memory)
}
