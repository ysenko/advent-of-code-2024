use std::{env, vec};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const SAFETY_MIN_DIFF: i32 = 1;
const SAFETY_MAX_DIFF: i32 = 3;

fn main() {
    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let reports = read_and_parse_reports(file_path).expect("Failed to read and parse file");

    println!("Reports found: {:?}", reports.len());

    println!("================= Part 1 =================");
    let safe_reports_count = reports
        .iter()
        .filter(|report| is_safe_report(&report))
        .count();
    println!("Safe reports count: {}", safe_reports_count);

    println!("================= Part 2 =================");
    let safe_reports_problem_dampener_count = reports
        .iter()
        .filter(|report| is_safe_report_problem_dampener(&report))
        .count();
    println!(
        "Safe reports count: {}",
        safe_reports_problem_dampener_count
    );
    for (idx, report) in reports.iter().enumerate() {
        let is_safe = is_safe_report(report);
        let is_tolerated_safe = is_safe_report_problem_dampener(report);
        if !is_safe && !is_tolerated_safe {
            println!("{} {}-{} - {:?}", idx, is_safe, is_tolerated_safe, report);
        }
    }
}

fn read_and_parse_reports(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut reports = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut levels = Vec::new();
        for level_str in line.split_whitespace() {
            match level_str.parse::<i32>() {
                Ok(level) => levels.push(level),
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid level: {}", level_str),
                    ))
                }
            }
        }
        if !levels.is_empty() {
            reports.push(levels)
        }
    }

    Ok(reports)
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    find_unsafe_level(report).is_none()
}

fn is_safe_report_problem_dampener(report: &Vec<i32>) -> bool {
    let unsafe_level = find_unsafe_level(report);
    match unsafe_level {
        Some((unsafe_lvl_1_candidate, unsafe_lvl_2_candidate)) => {
            let mut report_variant_1 = report.clone();
            report_variant_1.remove(unsafe_lvl_1_candidate);

            let mut report_variant_2 = report.clone();
            report_variant_2.remove(unsafe_lvl_2_candidate);

            let mut options = vec![
                report_variant_1, report_variant_2,
            ];

            if unsafe_lvl_1_candidate == 1 {
                let mut report_variant_3 = report.clone();
                report_variant_3.remove(0);
                options.push(report_variant_3);
            }

            for option in options {
                if find_unsafe_level(&option).is_none() {
                    return true;
                }
            }
            
            false
        }
        None => true,
    }
}

fn find_unsafe_level(report: &Vec<i32>) -> Option<(usize, usize)> {
    if report.len() < 2 {
        return None;
    }

    let first = report[0];
    let second = report[1];
    let growing = is_growing(first, second);

    for current_idx in 1..report.len() {
        let prev = report[current_idx - 1];
        let current = report[current_idx];

        if (prev - current).abs() < SAFETY_MIN_DIFF
            || (prev - current).abs() > SAFETY_MAX_DIFF
        {
            return Some((current_idx - 1, current_idx));
        }

        if is_growing(prev, current) != growing {
            return Some((current_idx - 1, current_idx));
        }
    }

    None
}

fn is_growing(x: i32, y: i32) -> bool {
    x < y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_unsafe_level_returns_none_on_safe_reports() {
        let safe_reports = vec![
            vec![1, 2, 3, 4, 5],
            vec![7, 6, 4, 2, 1],
            vec![1, 3, 6, 7, 9],
            vec![21, 22, 24, 25, 26, 27],
        ];

        for report in safe_reports {
            assert_eq!(find_unsafe_level(&report), None, "Report {:?} considered unsafe, but expected to be safe", report);
        }
    }

    #[test]
    fn test_find_unsafe_level_returns_returns_positions_for_unsafe_reports() {
        let unsafe_reports = vec![
            (vec![1, 2, 7, 8, 9], Some((1, 2))),
            (vec![9, 7, 6, 2, 1], Some((2, 3))),
            (vec![1, 3, 2, 4, 5], Some((1, 2))),
            (vec![8, 6, 4, 4, 1], Some((2, 3)))
        ];

        for (report, expected_positions) in unsafe_reports {
            assert_eq!(find_unsafe_level(&report), expected_positions, "Report {:?} considered unsafe, but expected to be safe", report);
        }
    }

    #[test]
    fn test_is_safe_report_problem_dampener_can_tolerate_unsafe_level() {
        let unsafe_reports = vec![
            vec![24, 21, 22, 24, 25, 26, 27],
        ];

        for report in unsafe_reports {
            assert_eq!(is_safe_report_problem_dampener(&report), true, "Report {:?} considered unsafe, but expected to be safe", report);
        }
    }
}
