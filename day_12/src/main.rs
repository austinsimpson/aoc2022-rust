use std::{str::FromStr, fmt::Display, collections::{HashSet, VecDeque}};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize
}

struct Matrix<T: Clone + Eq> {
    values: Vec<T>,
    rows: usize,
    cols: usize
}

impl<T: Clone + Eq> Matrix<T> {
    fn fill(rows: usize, cols: usize, value: T) -> Matrix<T> {
        let values: Vec<T> = vec![value; rows * cols];
        Matrix {
            values,
            rows,
            cols
        }
    }

    fn new(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T> {
        Matrix {
            values: data,
            rows,
            cols
        }
    }

    fn get(&self, coord: &Coordinate) -> &T {
        &self.values[coord.y * self.cols + coord.x]
    }

    fn set(&mut self, coord: &Coordinate, val: T) {
        self.values[coord.y * self.cols + coord.x] = val;
    }
}

struct HeightMap {
    values: Matrix<u8>,
    start: Coordinate,
    end: Coordinate 
}

impl FromStr for HeightMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::new();
        let mut row_count = 0usize;
        let mut col_count = 0usize;
        let mut start = Coordinate { x: 0, y: 0 };
        let mut end = Coordinate { x: 0, y: 0 }; 
        let lines = s.lines();

        for (line_idx, line) in lines.enumerate() {
            row_count += 1;
            col_count = line.len();

            for (char_idx, char) in line.chars().enumerate() {
                match char {
                    'S' => {
                        start.x = char_idx;
                        start.y = line_idx;
                        values.push(0)
                    },
                    'E' => {
                        end.x = char_idx;
                        end.y = line_idx;
                        values.push(25)
                    }, 
                    _ => {
                        let ordinal = char as u8 - 'a' as u8;
                        values.push(ordinal)
                    }
                }
            }
        }
        
        Ok(HeightMap { 
            values: Matrix::new(row_count, col_count, values), 
            start: start, 
            end: end 
        }) 
    }
}

enum Direction {
    Up, Right, Down, Left
}

struct NeighborsIter<'a> {
    origin: &'a Coordinate,
    height_map: &'a HeightMap,
    direction: Direction,
    at_end: bool,
    is_reverse: bool
}

impl<'a, 'b> NeighborsIter<'a> {
    fn new(origin: &'a Coordinate, height_map: &'a HeightMap, is_reverse: bool) -> NeighborsIter<'a> {
        NeighborsIter { origin:  origin, height_map: height_map, direction: Direction::Up, at_end: false, is_reverse }
    }

    fn get_coord_in_direction(&self, direction: &Direction) -> Option<Coordinate> {
        match direction {
            Direction::Up => {
                if self.origin.y == 0 {
                    return None;
                }
                Some(Coordinate { x: self.origin.x, y: self.origin.y - 1 })
            }
            Direction::Right => {
                if self.origin.x == self.height_map.values.cols - 1 {
                    return None
                }
                Some(Coordinate { x: self.origin.x + 1, y: self.origin.y })
            },
            Direction::Down => {
                if self.origin.y == self.height_map.values.rows - 1 {
                    return None   
                }
                Some(Coordinate { x: self.origin.x, y: self.origin.y + 1 })
            },
            Direction::Left => {
                if self.origin.x == 0 {
                    return None;
                }
                Some(Coordinate { x: self.origin.x - 1, y: self.origin.y })
            }
        }
    }
}

impl<'a, 'b> Iterator for NeighborsIter<'a> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let mut found = false;
        let mut result = None;
        
        let value_at_origin = *self.height_map.get(self.origin);
        
        while !found && !self.at_end {
            let new_coord = self.get_coord_in_direction(&self.direction);
            match self.direction {
                Direction::Up => self.direction = Direction::Right,
                Direction::Right => self.direction = Direction::Down,
                Direction::Down => self.direction = Direction::Left,
                Direction::Left => self.at_end = true
            }
            if new_coord.is_none() {
                continue;
            }

            let value_at_destination = *self.height_map.get(&new_coord.unwrap());
            let difference = value_at_destination as i64 - value_at_origin as i64;
            let is_valid_neighbor = match self.is_reverse {
                false => difference <= 1,
                true => difference >= -1 
            };
            
            if is_valid_neighbor {
                found = true;
                result = Some(new_coord.unwrap())
            }
        }
        result
    }
}

impl HeightMap {
    fn get(&self, coord: &Coordinate) -> &u8 {
        self.values.get(coord)
    }

    fn get_accessible_neighbors<'a>(&'a self, coord: &'a Coordinate, is_reverse: bool) -> NeighborsIter {
        NeighborsIter::new(coord, self, is_reverse)
    }

    fn shortest_path(&self, start: &Coordinate, end: &Coordinate) -> (Matrix<Option<u32>>, Option<u32>) {
        let mut distances: Matrix<Option<u32>> = Matrix::fill(self.values.rows, self.values.cols, None);
        distances.set(start, Some(0));

        let mut visited_coordinates: HashSet<Coordinate> = HashSet::new();
        let mut to_process: VecDeque<Coordinate> = VecDeque::new();
        to_process.push_back(*start);

        while !to_process.is_empty() {
            let coord = to_process.pop_front().unwrap();
            if visited_coordinates.contains(&coord) {
                continue;
            }

            let distance_to_current = distances.get(&coord).unwrap();
            let new_distance = distance_to_current + 1; 
            for neighbor in self.get_accessible_neighbors(&coord, false) {
                let distance_at_neighbor = distances.get(&neighbor);
                if distance_at_neighbor.is_none() || new_distance < distance_at_neighbor.unwrap() {
                    distances.set(&neighbor, Some(new_distance));
                } 
                to_process.push_back(neighbor);
            }

            visited_coordinates.insert(coord);
        }

        let distance_to_end = distances.get(end).clone();
        (distances, distance_to_end)
    }

    fn find_shortest_start(&self, end: &Coordinate) -> u32 {
        let mut distances: Matrix<Option<u32>> = Matrix::fill(self.values.rows, self.values.cols, None);
        distances.set(end, Some(0));

        let mut visited_coordinates: HashSet<Coordinate> = HashSet::new();
        let mut to_process: VecDeque<Coordinate> = VecDeque::new();
        to_process.push_back(*end);

        while !to_process.is_empty() {
            let coord = to_process.pop_front().unwrap();
            if visited_coordinates.contains(&coord) {
                continue;
            }

            let value_at_coord = *self.values.get(&coord);
            if value_at_coord == 0 {
                return distances.get(&coord).unwrap()
            }

            let distance_to_current = distances.get(&coord).unwrap();
            let new_distance = distance_to_current + 1; 
            for neighbor in self.get_accessible_neighbors(&coord, true) {
                let distance_at_neighbor = distances.get(&neighbor);
                if distance_at_neighbor.is_none() || new_distance < distance_at_neighbor.unwrap() {
                    distances.set(&neighbor, Some(new_distance));
                } 
                to_process.push_back(neighbor);
            }

            visited_coordinates.insert(coord);
        }
        0
    }
}

fn main() {
    let height_map = HeightMap::from_str(include_str!("input.txt")).unwrap();
    let (distance_map, shortest_distance) = height_map.shortest_path(&height_map.start, &height_map.end);
    let soonest_distance = height_map.find_shortest_start(&height_map.end);

    println!("{shortest_distance:?} {soonest_distance}");
}
