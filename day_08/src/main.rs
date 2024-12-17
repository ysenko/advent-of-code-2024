use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const EMPTY: char = '.';

const SCALE_FACTOR: f64 = 100.0;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let lines = read_input(file_path).expect("Failed to read and parse file");
    let antenna_groups = find_antenna_groups(&lines);

    let mut pairs: Vec<AntennaPair> = vec![];
    for group in antenna_groups.values() {
        for antennas in group.iter().combinations(2) {
            let pair = AntennaPair::new(&antennas[0], &antennas[1]);
            pairs.push(pair);
        }
    }

    println!("Found {} antenna groups", antenna_groups.len());
    println!("Found {} antenna pairs", pairs.len());
    println!("===================== Part 1 =====================");
    let antinodes = find_antinodes(&pairs, lines.len(), lines[0].len());
    for an in antinodes.iter() {
        println!("Antinode: {:?}", an);
    }

    println!("Found {} antinodes", antinodes.len());
    println!("===================== Part 2 =====================");
}

fn find_antinodes(pairs: &Vec<AntennaPair>, height: usize, width: usize) -> HashSet<Location> {
    let mut antinodes: HashSet<Location> = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            let loc = Location { x, y };
            if x == 6 && y == 6 {
                println!("Checking location: {:?}", loc);
            }
            for pair in pairs.iter() {
                if pair.is_antinode(&loc.clone()) {
                    antinodes.insert(loc);
                }
            }
        }
    }
    antinodes
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
    pub fn distance_to(&self, other: &Location) -> i64 {
        let x_diff = other.x as f64 - self.x as f64;
        let y_diff = other.y as f64 - self.y as f64;
        (((x_diff.powi(2) + y_diff.powi(2)).sqrt() * SCALE_FACTOR).round() / SCALE_FACTOR) as i64
    }
}

#[derive(Debug, Clone, Copy)]
struct Antenna {
    loc: Location,
    type_name: char,
}

struct AntennaPair<'a> {
    a: &'a Antenna,
    b: &'a Antenna,
}

impl AntennaPair<'_> {
    pub fn new<'a>(a: &'a Antenna, b: &'a Antenna) -> AntennaPair<'a> {
        AntennaPair { a, b }
    }

    fn distance<'a>(&self) -> i64 {
        self.b.loc.distance_to(&self.a.loc)
    }

    fn calculate_slope(&self) -> f64 {
        let x_diff = self.b.loc.x as f64 - self.a.loc.x as f64;
        let y_diff = self.b.loc.y as f64 - self.a.loc.y as f64;
        y_diff / x_diff
    }

    fn calculate_b(&self) -> f64 {
        let slope = self.calculate_slope();
        let b = (self.a.loc.y as f64) - (slope * self.a.loc.x as f64);
        b
    }

    fn is_on_line(&self, x: &Location) -> bool {
        let slope = self.calculate_slope();
        let b = self.calculate_b();
        let y = slope * x.x as f64 + b;
        y == x.y as f64
    }

    fn is_double_distanced(&self, loc: &Location) -> bool {
        let distance = self.distance();
        let double_dist = distance * 2;
        let dist_to_a = self.a.loc.distance_to(loc);
        let dist_to_b = self.b.loc.distance_to(loc);

        (double_dist == dist_to_a && distance == dist_to_b)
            || (double_dist == dist_to_b && distance == dist_to_a)
    }

    pub fn is_antinode(&self, loc: &Location) -> bool {
        self.is_on_line(loc) && self.is_double_distanced(loc)
    }
}

fn find_antenna_groups(map: &Vec<String>) -> HashMap<char, Vec<Antenna>> {
    let mut groups: HashMap<char, Vec<Antenna>> = HashMap::new();
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != EMPTY {
                let antenna = Antenna {
                    loc: Location { x, y },
                    type_name: c,
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

fn print_map(
    height: usize,
    width: usize,
    antinodes: &HashSet<Location>,
    antennas: &HashMap<Location, Antenna>,
) {
    for y in 0..height {
        for x in 0..width {
            let loc = Location { x, y };

            if let Some(a) = antennas.get(&loc) {
                print!("{}", a.type_name);
            } else if antinodes.contains(&loc) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;

    #[test]
    fn test_location_distance() {
        let a = Location { x: 4, y: 3 };
        let b = Location { x: 5, y: 5 };
        let expected_distance = (((5 as f64).sqrt() * SCALE_FACTOR).round() / SCALE_FACTOR) as i64;

        assert_eq!(a.distance_to(&b), expected_distance);
        assert_eq!(b.distance_to(&a), expected_distance);
    }

    #[test]
    fn test_is_on_line() {
        let a = Antenna {
            loc: Location { x: 4, y: 3 },
            type_name: 'A',
        };
        let b = Antenna {
            loc: Location { x: 5, y: 5 },
            type_name: 'A',
        };
        let pair = AntennaPair::new(&a, &b);

        let loc = Location { x: 3, y: 1 };
        assert_eq!(pair.is_on_line(&loc), true);

        let loc = Location { x: 6, y: 7 };
        assert_eq!(pair.is_on_line(&loc), true);

        let c = Antenna {
            loc: Location { x: 6, y: 5 },
            type_name: 'A',
        };
        let d = Antenna {
            loc: Location { x: 8, y: 8 },
            type_name: 'A',
        };
        let pair = AntennaPair::new(&c, &d);

        let loc = Location { x: 10, y: 11 };
        assert_eq!(pair.is_on_line(&loc), true);
    }

    #[test]
    fn test_is_not_on_line() {
        let a = Antenna {
            loc: Location { x: 4, y: 3 },
            type_name: 'A',
        };
        let b = Antenna {
            loc: Location { x: 5, y: 5 },
            type_name: 'A',
        };
        let pair = AntennaPair::new(&a, &b);

        let loc = Location { x: 2, y: 1 };
        assert_eq!(pair.is_on_line(&loc), false);
    }

    #[test]
    fn test_is_double_distanced() {
        // let a = Antenna {
        //     loc: Location { x: 4, y: 3 },
        //     type_name: 'A',
        // };
        // let b = Antenna {
        //     loc: Location { x: 5, y: 5 },
        //     type_name: 'A',
        // };
        // let pair = AntennaPair::new(&a, &b);
        // let loc = Location { x: 3, y: 1 };

        // assert_eq!(pair.is_double_distanced(&loc), true);

        let c = Antenna {
            loc: Location { x: 6, y: 5 },
            type_name: 'A',
        };
        let d = Antenna {
            loc: Location { x: 8, y: 8 },
            type_name: 'A',
        };
        let pair = AntennaPair::new(&c, &d);

        let loc = Location { x: 10, y: 11 };
        assert_eq!(pair.is_double_distanced(&loc), true);
    }

    #[test]
    fn test_find_antinodes() {
        let map = vec![
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];
        let antenna_groups = find_antenna_groups(&map);

        let mut pairs: Vec<AntennaPair> = vec![];
        let mut antennas_list: HashMap<Location, Antenna> = HashMap::new();
        for group in antenna_groups.values() {
            for antennas in group.iter().combinations(2) {
                let pair = AntennaPair::new(&antennas[0], &antennas[1]);
                pairs.push(pair);
                antennas_list.insert(antennas[0].loc, *antennas[0]);
                antennas_list.insert(antennas[1].loc, *antennas[1]);
            }
        }

        let antinodes = find_antinodes(&pairs, map.len(), map[0].len());
        print_map(map.len(), map[0].len(), &antinodes, &antennas_list);

        assert_eq!(antinodes.len(), 14);
    }
}
