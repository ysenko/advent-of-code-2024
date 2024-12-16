use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let lines = read_input(file_path).expect("Failed to read and parse file");
    let equations: Vec<Equation> = lines
        .iter()
        .map(|line| Equation::from_str(line))
        .collect::<Result<Vec<Equation>, _>>()
        .expect("Failed to parse equations");

    println!("Read {} equations from input file", equations.len());
    println!("===================== Part 1 =====================");
    let sum_of_values: i64 = equations
        .iter()
        .filter(|&e| Solver::new(e).solve().is_some())
        .map(|e| e.result)
        .sum();

    println!("Sum of values: {}", sum_of_values);

    println!("===================== Part 2 =====================");
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = vec![];

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}

#[derive(Debug)]
struct Equation {
    result: i64,
    values: Vec<i64>,
}

impl Equation {
    pub fn from_str(eq_str: &str) -> Result<Equation, String> {
        // Split the input string by the colon
        let parts: Vec<&str> = eq_str.split(':').collect();

        // Check if we have exactly two parts
        if parts.len() != 2 {
            return Err("Invalid format".to_string());
        }

        // Parse the result part
        let result = parts[0]
            .trim()
            .parse::<i64>()
            .map_err(|_| "Invalid result number".to_string())?;

        // Parse the values part
        let values: Result<Vec<i64>, _> = parts[1]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect();

        // Check if parsing values was successful
        let values = values.map_err(|_| "Invalid values".to_string())?;

        // Return the Equation instance
        Ok(Equation { result, values })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Ops {
    Add,
    Mul,
    Concat,
}

impl Ops {
    fn variants() -> Vec<Ops> {
        vec![Ops::Add, Ops::Mul, Ops::Concat]
    }
}

struct Solver<'a> {
    eq: &'a Equation,
}

impl<'a> Solver<'a> {
    fn new(eq: &'a Equation) -> Solver<'a> {
        Solver { eq }
    }

    fn solve(&self) -> Option<Vec<Ops>> {
        for ops_sequence in self.get_operations() {
            if self.eq.result == self.apply_operations(&ops_sequence) {
                println!("Found solution: {:?}", ops_sequence);
                return Some(ops_sequence);
            }
        }
        None
    }

    fn get_operations(&self) -> Vec<Vec<Ops>> {
        let number_of_ops = self.eq.values.len() - 1;
        let variants = Ops::variants();
        let iterators: Vec<_> = std::iter::repeat(variants.iter().cloned())
            .take(number_of_ops)
            .collect();
        iterators.into_iter().multi_cartesian_product().collect()
    }

    fn apply_operations(&self, ops: &Vec<Ops>) -> i64 {
        let mut result = self.eq.values[0];
        for (op, value) in ops.iter().zip(self.eq.values.iter().skip(1)) {
            match op {
                Ops::Add => result += value,
                Ops::Mul => result *= value,
                Ops::Concat => {
                    let res = result.to_string() + value.to_string().as_str();
                    result = res.parse::<i64>().unwrap();
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_apply_concat() {
        let eq = super::Equation {
            result: 156,
            values: vec![15, 6],
        };
        let solver = super::Solver::new(&eq);
        let ops = vec![super::Ops::Concat];
        assert_eq!(solver.apply_operations(&ops), 156);
    }

    #[test]
    fn solve_with_concat() {
        let eq = super::Equation {
            result: 156,
            values: vec![15, 6],
        };
        let solver = super::Solver::new(&eq);
        let ops = solver.solve();
        assert_eq!(ops, Some(vec![super::Ops::Concat]));
    }
}
