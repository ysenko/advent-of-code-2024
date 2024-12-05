
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // Open the file and read its content
    let (mut col1, mut col2) = read_and_parse_file(file_path).expect("Failed to read and parse file");

    // Print the vectors
    println!("Column 1: length {:?}", col1.len());
    println!("Column 2: length {:?}", col2.len());

    col1.sort();
    col2.sort();

    println!("==================== Part 1 ====================");
    println!("Total distance: {}", total_distance(&col1, &col2));

    println!("==================== Part 1 ====================");
    println!("Similarity score: {}", get_similarity_score(col1, col2));
}

fn read_and_parse_file(file_path: &str) -> io::Result<(Vec<u32>, Vec<u32>)> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut col1: Vec<u32> = Vec::new();
    let mut col2: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (numbers[0].parse::<u32>(), numbers[1].parse::<u32>()) {
                col1.push(num1);
                col2.push(num2);
            }
        }
    }

    Ok((col1, col2))
}

fn total_distance(places_1: &Vec<u32>, places_2: &Vec<u32>) -> u64 {
    (0..places_1.len() as usize)
        .map(|i| {
            let place_1 = places_1[i] as i64;
            let place_2 = places_2[i] as i64;
            let distance = (place_1 - place_2).abs() as u64;
            distance
        })
        .sum()
}

fn get_similarity_score(places_1: Vec<u32>, places_2: Vec<u32>) -> u32 {
    places_1.iter().map(|number| *number * count_number(*number, &places_2)).sum()
}

fn count_number(number: u32, places: &Vec<u32>) -> u32 {
    places.iter().filter(|&place| *place == number).count() as u32
}