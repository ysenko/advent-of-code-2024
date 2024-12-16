use std::collections::{HashMap, HashSet};
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

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Guard {
    position: MapPosition,
    direction: Direction,
    start_position: MapPosition,
}

impl Guard {
    pub fn new(position: MapPosition, direction: Direction, start_position: MapPosition) -> Guard {
        Guard {
            position,
            direction,
            start_position,
        }
    }
    fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }

    fn make_move(&mut self, map: &Map) -> Option<MapPosition> {
        if self.direction == Direction::Up && self.position.y == 0 {
            return None;
        } else if self.direction == Direction::Down && self.position.y == map.height - 1 {
            return None;
        } else if self.direction == Direction::Left && self.position.x == 0 {
            return None;
        } else if self.direction == Direction::Right && self.position.x == map.width - 1 {
            return None;
        }

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
    let (map, guard) = read_map_data().expect("Failed to read map");
    // println!("{:?}", map);
    println!("{:?}", guard);

    let map_2 = map.clone();
    let guard_2 = guard.clone();

    println!("==================== Part 1 ====================");

    let mut tracker = GuardTracker::new(&map, &guard);
    if let Some(visited_positions) = tracker.track() {
        println!(
            "Guard visited {} unique position(s)",
            visited_positions.len() + 1
        );
        // print_map(&map, &guard, visited_positions);
    } else {
        println!("Guard made a loop");
    }

    println!("==================== Part 2 ====================");
    let mut tracker = GuardTracker::new(&map_2, &guard_2);
    let loops_count = tracker.find_loops();
    println!("Guard made {} loop(s)", loops_count);
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
                    guard = Some(Guard::new(
                        MapPosition { x: pos_x, y: pos_y },
                        Direction::Up,
                        MapPosition { x: pos_x, y: pos_y },
                    ));
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

fn print_map(map: &Map, guard: &Guard, visited: HashMap<MapPosition, Direction>) {
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = MapPosition { x, y };
            if map.obstacles.contains(&pos) {
                print!("#");
            } else if guard.position == pos {
                print!("^");
            } else if let Some(direction) = visited.get(&pos) {
                if direction == &Direction::Up || direction == &Direction::Down {
                    print!("|");
                } else {
                    print!("-");
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

struct GuardTracker {
    map: Map,
    guard: Guard,
}

impl GuardTracker {
    pub fn new(map: &Map, guard: &Guard) -> GuardTracker {
        GuardTracker {
            map: map.clone(),
            guard: guard.clone(),
        }
    }

    pub fn track(&mut self) -> Option<HashMap<MapPosition, Direction>> {
        let mut visited = HashMap::new();
        visited.insert(self.guard.position.clone(), self.guard.direction);

        while let Some(new_position) = self.guard.make_move(&self.map) {
            if let Some(past_direction) = visited.get(&new_position) {
                if *past_direction == self.guard.direction {
                    return None;
                }
            }
            visited.insert(new_position.clone(), self.guard.direction);
        }
        Some(visited)
    }

    fn predict_next_guard_location(&self) -> Option<MapPosition> {
        let mut guard = self.guard.clone();
        guard.make_move(&self.map)
    }

    fn put_obstacle(&mut self, position: Option<MapPosition>) -> Option<Map> {
        if let Some(obstacle_position) = position {
            if obstacle_position == self.guard.start_position {
                return None;
            }
            let mut updated_map = self.map.clone();
            updated_map.obstacles.insert(obstacle_position);
            return Some(updated_map);
        }
        None
    }

    pub fn find_loops(&mut self) -> u32 {
        let mut added_obstacle_positions = HashSet::new();
        loop {
            let next_position = self.predict_next_guard_location();
            if let Some(new_map) = self.put_obstacle(next_position.clone()) {
                if GuardTracker::new(&new_map, &self.guard).track().is_none() {
                    added_obstacle_positions.insert(next_position.unwrap());
                }
            }
            if self.guard.make_move(&self.map).is_none() {
                break;
            }
        }
        added_obstacle_positions.len() as u32
    }
}
