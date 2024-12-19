use itertools::{Itertools, chain};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::{max, min};

const EMPTY: char = '.';

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let lines = read_input(file_path).expect("Failed to read and parse file");
    let max_y = lines.len();
    let max_x = lines[0].len();
    let antenna_groups = find_antenna_groups(&lines);

    let mut antennas_list: HashMap<Location, Antenna> = HashMap::new();
    let mut antinodes = HashSet::new();
    let mut resonant_harmonic_antinodes: HashSet<Location> = HashSet::new();
    for group in antenna_groups.values() {
        for antennas in group.iter().combinations(2) {
            let pair = AntennaPair::new(&antennas[0], &antennas[1]).unwrap();
            let pair_antinodes = pair.get_antinodes();
            for antinode in remove_unreachable_locations(max_x, max_y, &pair_antinodes) {
                antinodes.insert(antinode.clone());
            }
            antennas_list.insert(antennas[0].loc, *antennas[0]);
            antennas_list.insert(antennas[1].loc, *antennas[1]);

            resonant_harmonic_antinodes.extend(pair.get_resonant_harmonics_antinodes(max_x, max_y));
        }
    }

    println!("Found {} antenna groups", antenna_groups.len());
    println!("===================== Part 1 =====================");
    println!("Found {} antinodes", antinodes.len());

    println!("===================== Part 2 =====================");
    println!("Found {} antinodes", resonant_harmonic_antinodes.into_iter().count());
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    pub fn new(x: i32, y: i32) -> Result<Location, String> {
        if x < 0 || y < 0 {
            Err("Location coordinates must be positive".to_string())
        } else {
            Ok(Location { x: x as usize, y: y as usize })
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Distance {
    x_diff: i32,
    y_diff: i32,
}

impl Distance {
    pub fn from_location(a: &Location, b: &Location) -> Distance {
        Distance {
            x_diff: a.x as i32 - b.x as i32,
            y_diff: a.y as i32 - b.y as i32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Antenna {
    loc: Location,
    frequency: char,
}

struct AntennaPair<'a> {
    a: &'a Antenna,
    b: &'a Antenna,
}

impl AntennaPair<'_> {
    pub fn new<'a>(a: &'a Antenna, b: &'a Antenna) -> Result<AntennaPair<'a>, String> {
        if a.frequency != b.frequency {
            Err("Antennas must be of the frequency".to_string())
        } else {
            Ok(AntennaPair { a, b })
        }
    }

    fn calculate_slope(&self) -> f64 {
        let x_diff = self.b.loc.x as f64 - self.a.loc.x as f64;
        let y_diff = self.b.loc.y as f64 - self.a.loc.y as f64;
        y_diff / x_diff
    }
    fn calculate_b(&self) -> f64 {
        let slope = self.calculate_slope();
        (self.a.loc.y as f64) - (slope * self.a.loc.x as f64)
    }

    pub fn get_antinodes(&self) -> Vec<Location> {
        let dist = Distance::from_location(&self.a.loc, &self.b.loc);
        vec![
            Location::new(
                self.a.loc.x as i32 + dist.x_diff,
                self.a.loc.y as i32 + dist.y_diff,
            ),
            Location::new(
                self.a.loc.x as i32 - dist.x_diff,
                self.a.loc.y as i32 - dist.y_diff,
            ),
            Location::new(
                self.b.loc.x as i32 + dist.x_diff,
                self.b.loc.y as i32 + dist.y_diff,
            ),
            Location::new(
                self.b.loc.x as i32 - dist.x_diff,
                self.b.loc.y as i32 - dist.y_diff,
            ),
        ]
        .into_iter()
        .filter(|loc_result| loc_result.is_ok())
        .map(|loc_result| loc_result.unwrap())
        .filter(|&loc| loc != self.a.loc && loc != self.b.loc)
        .collect()
    }

    pub fn get_resonant_harmonics_antinodes(&self, max_x: usize, max_y: usize) -> Vec<Location> {
        let mut antinodes = vec![];
        let slope = self.calculate_slope();
        let b = self.calculate_b();

        for x in 0..max_x {
            let y = slope * x as f64 + b;
            let loc_y = y as usize;
            if (y - loc_y as f64) == 0.0 && (loc_y < max_y) {
                antinodes.push(Location::new(x as i32, loc_y as i32).unwrap());
            }
        }
        antinodes
    }
}

fn find_antenna_groups(map: &Vec<String>) -> HashMap<char, Vec<Antenna>> {
    let mut groups: HashMap<char, Vec<Antenna>> = HashMap::new();
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != EMPTY {
                let antenna = Antenna {
                    loc: Location { x, y },
                    frequency: c,
                };
                match groups.get_mut(&c) {
                    Some(antennas) => {
                        antennas.push(antenna);
                    }
                    None => {
                        groups.insert(c, vec![antenna]);
                    }
                }
            }
        }
    }
    groups
}

fn remove_unreachable_locations(
    max_x: usize,
    max_y: usize,
    locations: &Vec<Location>,
) -> Vec<Location> {
    locations
        .iter()
        .filter(|&loc| loc.x < max_x && loc.y < max_y)
        .cloned()
        .collect()
}

fn print_map(
    height: usize,
    width: usize,
    antinodes: &HashSet<Location>,
    antennas: &HashMap<Location, Antenna>,
) {
    for y in 0..height {
        for x in 0..width {
            let loc = Location { x, y };

            if antinodes.contains(&loc) {
                print!("#");
            } else if let Some(a) = antennas.get(&loc) {
                print!("{}", a.frequency);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_slope() {
        let a = Location::new(7, 7).unwrap();
        let b = Location::new(8, 8 ).unwrap();
        let antenna_a = Antenna{loc: a, frequency: 'a'};
        let antenna_b = Antenna{loc: b, frequency: 'a'};
        let pair = AntennaPair::new(&antenna_a, &antenna_b).unwrap();

        assert_eq!(pair.calculate_slope(), 1.0);
    }

    #[test]
    fn test_calculate_b() {
        let a = Location::new(7, 7).unwrap();
        let b = Location::new(8, 8 ).unwrap();
        let antenna_a = Antenna{loc: a, frequency: 'a'};
        let antenna_b = Antenna{loc: b, frequency: 'a'};
        let pair = AntennaPair::new(&antenna_a, &antenna_b).unwrap();

        assert_eq!(pair.calculate_b(), 0.0);
    }

    #[test]
    fn test_find_antinodes() {
        let map = vec![
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];
        let antenna_groups = find_antenna_groups(&map);
        let max_y = map.len();
        let max_x = map[0].len();

        let mut antinodes = HashSet::new();
        for group in antenna_groups.values() {
            for antennas in group.iter().combinations(2) {
                let pair = AntennaPair::new(&antennas[0], &antennas[1]).unwrap();
                let pair_antinodes = pair.get_resonant_harmonics_antinodes(max_x, max_y);
                antinodes.extend(pair_antinodes);
            }
        }

        println!("{:?}", antinodes);
        assert_eq!(antinodes.len(), 12);
    }
}
