use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const WORD_PART_1: &str = "XMAS";
const WORD_PART_2: &str = "MAS";

#[derive(Debug, PartialEq)]
enum Direction {
    Right((usize, usize)),
    Left((usize, usize)),
    Down((usize, usize)),
    Up((usize, usize)),
    DiagonalRightDown((usize, usize)),
    DiagonalRightUp((usize, usize)),
    DiagonalLeftDown((usize, usize)),
    DiagonalLeftUp((usize, usize)),
}

fn main() {
    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let words_data = read_words_data(file_path).expect("Failed to read and parse file");

    println!("===================== Part 1 =====================");
    let all_words = find_all_words(WORD_PART_1, &words_data);
    println!("Words found: {:?}", all_words.len());

    println!("===================== Part 2 =====================");
    let crossed_words = find_crossed_words(WORD_PART_2, &words_data);
    println!("Crossed words found: {:?}", crossed_words.len());
}

fn read_words_data(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut words = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            words.push(line);
        }
    }
    Ok(words)
}

fn find_all_words(word: &str, words_data: &Vec<String>) -> Vec<Direction> {
    let mut result: Vec<Direction> = Vec::new();
    for (row, row_data) in words_data.iter().enumerate() {
        for (col, _) in row_data.chars().enumerate() {
            for direction in vec![
                Direction::Right((row, col)),
                Direction::Left((row, col)),
                Direction::Down((row, col)),
                Direction::Up((row, col)),
                Direction::DiagonalRightDown((row, col)),
                Direction::DiagonalRightUp((row, col)),
                Direction::DiagonalLeftDown((row, col)),
                Direction::DiagonalLeftUp((row, col)),
            ] {
                if let Some(w) = get_word(word.len(), &direction, words_data) {
                    if w == word {
                        result.push(direction);
                    }
                }
            }
        }
    }
    result
}

fn find_crossed_words(word: &str, words_data: &Vec<String>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for (row, row_data) in words_data.iter().enumerate() {
        for (col, _) in row_data.chars().enumerate() {
            let crossing_words = get_words_crossed_at_position(word.len(), row, col, words_data);
            if crossing_words
                .iter()
                .map(|w| if w == WORD_PART_2 { 1 } else { 0 })
                .sum::<i32>()
                > 1
            {
                result.push((row, col));
            }
        }
    }

    result
}

fn get_word(len: usize, direction: &Direction, words_data: &Vec<String>) -> Option<String> {
    let len = len;

    match direction {
        Direction::Right((row, col)) => {
            if col + (len - 1) < words_data[*row].len() {
                Some(words_data[*row][*col..*col + len].to_string())
            } else {
                None
            }
        }
        Direction::Left((row, col)) => {
            if *col >= (len - 1) {
                Some(
                    words_data[*row][col - (len - 1)..*col + 1]
                        .chars()
                        .rev()
                        .collect::<String>(),
                )
            } else {
                None
            }
        }
        Direction::Down((row, col)) => {
            if row + (len - 1) < words_data.len() {
                Some(
                    (0..len)
                        .map(|i| words_data[row + i].chars().nth(*col).unwrap())
                        .collect::<String>(),
                )
            } else {
                None
            }
        }
        Direction::Up((row, col)) => {
            if *row >= (len - 1) {
                Some(
                    (0..len)
                        .map(|i| words_data[row - i].chars().nth(*col).unwrap())
                        .collect::<String>(),
                )
            } else {
                None
            }
        }
        Direction::DiagonalRightDown((row, col)) => {
            if row + len - 1 < words_data.len() && col + len - 1 < words_data[0].len() {
                Some(
                    (0..len)
                        .map(|i| words_data[row + i].chars().nth(col + i).unwrap())
                        .collect::<String>(),
                )
            } else {
                None
            }
        }
        Direction::DiagonalRightUp((row, col)) => {
            if *row >= len - 1 && col + len - 1 < words_data[0].len() {
                Some(
                    (0..len)
                        .map(|i| words_data[row - i].chars().nth(col + i).unwrap())
                        .collect::<String>(),
                )
            } else {
                None
            }
        }
        Direction::DiagonalLeftDown((row, col)) => {
            if row + len - 1 < words_data.len() && *col >= (len - 1) {
                Some(
                    (0..len)
                        .map(|i| words_data[row + i].chars().nth(col - i).unwrap())
                        .collect::<String>(),
                )
            } else {
                None
            }
        }
        Direction::DiagonalLeftUp((row, col)) => {
            if *row >= (len - 1) && *col >= (len - 1) {
                Some(
                    (0..len)
                        .map(|i| words_data[row - i].chars().nth(col - i).unwrap())
                        .collect::<String>(),
                )
            } else {
                None
            }
        }
    }
}

fn get_words_crossed_at_position(
    len: usize,
    row: usize,
    col: usize,
    words_data: &Vec<String>,
) -> Vec<String> {
    let mut words = vec![];
    if len % 2 == 0 {
        return words;
    }

    let half_len = len / 2;
    let max_row = words_data.len();
    let max_col = words_data[0].len();

    // Diagonal
    if col >= half_len && col + half_len < max_col && row >= half_len && row + half_len < max_row {
        if let Some(word) = get_word(
            len,
            &&Direction::DiagonalRightDown((row - half_len, col - half_len)),
            words_data,
        ) {
            words.push(word);
        }
        if let Some(word) = get_word(
            len,
            &&Direction::DiagonalLeftUp((row + half_len, col + half_len)),
            words_data,
        ) {
            words.push(word);
        }
        if let Some(word) = get_word(
            len,
            &&Direction::DiagonalLeftDown((row - half_len, col + half_len)),
            words_data,
        ) {
            words.push(word);
        }
        if let Some(word) = get_word(
            len,
            &&Direction::DiagonalRightUp((row + half_len, col - half_len)),
            words_data,
        ) {
            words.push(word);
        }
    }

    words
}
#[cfg(test)]
mod test {
    use crate::{get_words_crossed_at_position, Direction};

    #[test]
    fn test_find_word_right() {
        let words_data_1: Vec<String> = vec![
            "XMAS.".to_string(),
            ".....".to_string(),
            ".....".to_string(),
            ".....".to_string(),
        ];
        let words_data_2: Vec<String> = vec![
            ".XMAS".to_string(),
            ".....".to_string(),
            ".....".to_string(),
            ".....".to_string(),
        ];
        let word = "XMAS";
        let result_1 = super::find_all_words(word, &words_data_1);
        let result_2 = super::find_all_words(word, &words_data_2);

        assert_eq!(result_1, vec![Direction::Right((0, 0))]);
        assert_eq!(result_2, vec![Direction::Right((0, 1))]);
    }

    #[test]
    fn test_find_word_left() {
        let words_data_1: Vec<String> = vec![
            "SAMX.".to_string(),
            ".....".to_string(),
            ".....".to_string(),
            ".....".to_string(),
        ];
        let words_data_2: Vec<String> = vec![
            ".SAMX".to_string(),
            ".....".to_string(),
            ".....".to_string(),
            ".....".to_string(),
        ];
        let word = "XMAS";
        let result_1 = super::find_all_words(word, &words_data_1);
        let result_2 = super::find_all_words(word, &words_data_2);

        assert_eq!(result_1, vec![Direction::Left((0, 3))]);
        assert_eq!(result_2, vec![Direction::Left((0, 4))]);
    }

    #[test]
    fn test_find_word_down() {
        let words_data_1: Vec<String> = vec![
            "X....".to_string(),
            "M....".to_string(),
            "A....".to_string(),
            "S....".to_string(),
            ".....".to_string(),
        ];
        let words_data_2: Vec<String> = vec![
            ".....".to_string(),
            "X....".to_string(),
            "M....".to_string(),
            "A....".to_string(),
            "S....".to_string(),
        ];
        let word = "XMAS";
        let result_1 = super::find_all_words(word, &words_data_1);
        let result_2 = super::find_all_words(word, &words_data_2);

        assert_eq!(result_1, vec![Direction::Down((0, 0))]);
        assert_eq!(result_2, vec![Direction::Down((1, 0))]);
    }

    #[test]
    fn test_find_word_up() {
        let words_data_1: Vec<String> = vec![
            "S....".to_string(),
            "A....".to_string(),
            "M....".to_string(),
            "X....".to_string(),
            ".....".to_string(),
        ];
        let words_data_2: Vec<String> = vec![
            ".....".to_string(),
            "S....".to_string(),
            "A....".to_string(),
            "M....".to_string(),
            "X....".to_string(),
        ];
        let word = "XMAS";
        let result_1 = super::find_all_words(word, &words_data_1);
        let result_2 = super::find_all_words(word, &words_data_2);

        assert_eq!(result_1, vec![Direction::Up((3, 0))]);
        assert_eq!(result_2, vec![Direction::Up((4, 0))]);
    }

    #[test]
    fn test_get_words_crossed_at_position() {
        let words_data_1: Vec<String> = vec![
            "XMAS.".to_string(),
            ".....".to_string(),
            ".....".to_string(),
            ".....".to_string(),
        ];
        let res = get_words_crossed_at_position(3, 0, 0, &words_data_1);
        let expected_res: Vec<String> = vec![];

        assert_eq!(res, expected_res);
    }
}
