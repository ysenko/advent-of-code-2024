use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum MapElement {
    Guard,
    Empty,
    Obstacle,
}

impl MapElement {
    fn from_char(c: char) -> Result<MapElement, String> {
        match c {
            '^' => Ok(MapElement::Guard),
            '.' => Ok(MapElement::Empty),
            '#' => Ok(MapElement::Obstacle),
            _ => Err(format!("Invalid map element: {}", c)),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MapPosition {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    height: usize,
    width: usize,
    obstacles: HashSet<MapPosition>,
}

impl Map {
    fn new(height: usize, width: usize, obstacles: HashSet<MapPosition>) -> Map {
        Map {
            height,
            width,
            obstacles,
        }
    }
}

#[derive(Debug)]
struct Guard {
    position: MapPosition,
    direction: Direction,
}

impl Guard {
    fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }

    pub fn make_move(&mut self, map: &Map) -> Option<MapPosition> {
        let new_position = match self.direction {
            Direction::Up => MapPosition {
                x: self.position.x,
                y: self.position.y - 1,
            },
            Direction::Down => MapPosition {
                x: self.position.x,
                y: self.position.y + 1,
            },
            Direction::Left => MapPosition {
                x: self.position.x - 1,
                y: self.position.y,
            },
            Direction::Right => MapPosition {
                x: self.position.x + 1,
                y: self.position.y,
            },
        };

        if new_position.x >= map.width || new_position.y >= map.height {
            return None;
        }

        if map.obstacles.contains(&new_position) {
            self.turn_right();
            return self.make_move(map);
        }

        self.position = new_position.clone();
        Some(new_position)
    }
}

fn main() {
    let (map, mut guard) = read_map_data().expect("Failed to read map");
    // println!("{:?}", map);
    println!("{:?}", guard);

    println!("==================== Part 1 ====================");
    let mut visited_positions = HashSet::new();
    loop {
        if let Some(new_position) = guard.make_move(&map) {
            println!("Guard moved to {:?}", new_position);
            visited_positions.insert(new_position);

            // print_map(&map, &guard);
        } else {
            break;
        }
    }
    println!("Guard made {} steps", visited_positions.len() + 1);
    print_map(&map, &guard, Some(visited_positions));
}

fn read_map_data() -> Result<(Map, Guard), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let map_raw = read_input(file_path).expect("Failed to read and parse file");

    let height = map_raw.len();
    let width = map_raw[0].len();
    let mut obstacles = HashSet::new();
    let mut guard: Option<Guard> = None;

    for (pos_y, map_line) in map_raw.iter().enumerate() {
        for (pos_x, map_element_raw) in map_line.chars().enumerate() {
            match MapElement::from_char(map_element_raw)? {
                MapElement::Guard => {
                    guard = Some(Guard {
                        position: MapPosition { x: pos_x, y: pos_y },
                        direction: Direction::Up,
                    });
                }
                MapElement::Empty => {}
                MapElement::Obstacle => {
                    obstacles.insert(MapPosition { x: pos_x, y: pos_y });
                }
            }
        }
    }
    if let Some(actual_guard) = guard {
        Ok((Map::new(height, width, obstacles), actual_guard))
    } else {
        Err("No guard found".to_string())
    }
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut input_lines = vec![];

    for line in reader.lines() {
        let line = line?;
        input_lines.push(line);
    }

    Ok(input_lines)
}

fn print_map(map: &Map, guard: &Guard, visited: Option<HashSet<MapPosition>>) {
    let visited = visited.unwrap_or(HashSet::new());

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = MapPosition { x, y };
            if map.obstacles.contains(&pos) {
                print!("#");
            } else if guard.position == pos {
                print!("^");
            } else if visited.contains(&pos) {
                print!("1");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
