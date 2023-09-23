use std::str::FromStr;

//Mainly used as a throwaway type because we really don't have to worry about invalid format.
#[derive(Debug)]
struct GenericParseErr;

#[derive(Debug)]
struct Orchard {
    rows: usize,
    columns: usize,
    data: Vec<u8>
}

impl FromStr for Orchard {
    type Err = GenericParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let number_of_rows = lines.len();
        let number_of_columns = lines[0].len();

        let mut data = Vec::new();
        data.reserve(number_of_rows * number_of_columns);

        for line in lines.iter() {
            for char in line.chars() {
                data.push(char.to_digit(10).unwrap() as u8)
            }  
        }

        Ok(Orchard { rows: number_of_rows, columns: number_of_columns, data: data })
    }
}

impl Orchard {
    fn get_value(&self, row_idx: usize, col_idx: usize) -> u8 {
        let index = row_idx * self.columns + col_idx;

        self.data[index]
    }

    fn rows_iter(&self) -> OrchardIter {
        OrchardIter {
            direction: OrchardIterDirection::Row,
            orchard: self,
            index: 0
        }
    }

    fn columns_iter(&self) -> OrchardIter {
        OrchardIter {
            direction: OrchardIterDirection::Column,
            orchard: self,
            index: 0
        }
    }

    fn get_neighbors(&self, row_idx: usize, col_idx: usize, orientation: OrchardIterDirection, direction: TreeNeighborsIterDirection) -> TreeNeighborsIter {
        TreeNeighborsIter { orchard: self, orientation: orientation, direction: direction, row_idx: row_idx as i8, col_idx: col_idx as i8 }
    }
}

#[derive(Clone, Copy)]
enum OrchardIterDirection {
    Row,
    Column
}

struct OrchardIter<'a> {
    orchard: &'a Orchard,
    direction: OrchardIterDirection, 
    index: usize
}

impl<'a> Iterator for OrchardIter<'a> {
    type Item = OrchardVecIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (fixed_index, lower_index, upper_index, max) = match self.direction {
            OrchardIterDirection::Row => (self.index, None, self.orchard.columns, self.orchard.rows),
            OrchardIterDirection::Column => (self.index, None, self.orchard.rows, self.orchard.columns)
        };

        if self.index >= max {
            return None;
        }

        let result = Some(OrchardVecIter {
            orchard: self.orchard,
            fixed_index: fixed_index,
            lower_index: lower_index,
            upper_index: upper_index,
            direction: match self.direction {
                OrchardIterDirection::Row => OrchardIterDirection::Column,
                OrchardIterDirection::Column => OrchardIterDirection::Row
            }
        });
        self.index = self.index + 1;
        
        result
    }
}

//This class is responsible for iterating along a row or column vector
struct OrchardVecIter<'a> {
    orchard: &'a Orchard,
    fixed_index: usize,
    lower_index: Option<usize>,
    upper_index: usize,
    direction: OrchardIterDirection
}

impl<'a> Iterator for OrchardVecIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {  

        self.lower_index = match self.lower_index {
            Some(idx) => Some(idx + 1),
            None => Some(0)
        };

        if self.lower_index.unwrap() == self.upper_index {
            return None
        }

        let (row_index, column_index) = match self.direction {
            OrchardIterDirection::Row => (self.lower_index.unwrap(), self.fixed_index),
            OrchardIterDirection::Column => (self.fixed_index, self.lower_index.unwrap())
        };

        let result = Some(self.orchard.get_value(row_index, column_index));
        result
    } 
}

impl<'a> DoubleEndedIterator for OrchardVecIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if (self.lower_index.is_some() && self.lower_index.unwrap() == self.upper_index) || self.upper_index == 0 {
            return None;
        }

        self.upper_index = self.upper_index - 1;
        let (row_index, column_index) = match self.direction {
            OrchardIterDirection::Row => (self.upper_index, self.fixed_index),
            OrchardIterDirection::Column => (self.fixed_index, self.upper_index)
        };

        let result = Some(self.orchard.get_value(row_index, column_index));
        result
    }
}

fn max_topography_indicator(running_max_opt: &mut Option<u8>, current_val: u8) -> Option<bool> {    
    match running_max_opt {
        None => {
            *running_max_opt = Some(current_val);
            return Some(true);
        },
        Some(running_max) => {
            if *running_max < current_val {
                *running_max_opt = Some(current_val);
                return Some(true);
            }
        }
    }
    Some(false)
}

#[derive(Clone, Copy)]
enum TreeNeighborsIterDirection {
    Forward,
    Backward
}

struct TreeNeighborsIter<'a> {
    orchard: &'a Orchard,
    orientation: OrchardIterDirection,
    direction: TreeNeighborsIterDirection,
    row_idx: i8,
    col_idx: i8
}

impl<'a> Iterator for TreeNeighborsIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.orientation, self.direction) { 
            (OrchardIterDirection::Row, TreeNeighborsIterDirection::Forward) => {
                self.col_idx += 1;
            },
            (OrchardIterDirection::Row, TreeNeighborsIterDirection::Backward) => {
                self.col_idx -= 1;
            },
            (OrchardIterDirection::Column, TreeNeighborsIterDirection::Forward) => {
                self.row_idx += 1;
            },
            (OrchardIterDirection::Column, TreeNeighborsIterDirection::Backward) => {
                self.row_idx -= 1;
            }
        }
        
        if self.row_idx < 0 || self.row_idx >= self.orchard.rows as i8 || self.col_idx < 0 || self.col_idx >= self.orchard.columns as i8 {
            return None;
        }

        let value = self.orchard.get_value(self.row_idx as usize, self.col_idx as usize);
        Some(value)
    }
}


fn get_visible_tree_count(height: u8, neighbors_iter: TreeNeighborsIter) -> i32 {
    let mut total = 0;
    for neighbor in neighbors_iter {
        total += 1;
        if neighbor >= height {
            break;
        }
    }
    total
}

fn main() {
    let orchard = Orchard::from_str(include_str!("input.txt")).unwrap();

    // Part 1
    let mut tree_visibility = vec![false; orchard.rows * orchard.columns];
    for (row_idx, row) in orchard.rows_iter().enumerate() {
        for (col_idx, cell) in row.scan(None, max_topography_indicator).enumerate() {
            tree_visibility[row_idx * orchard.columns + col_idx] = cell
        }
    }
    
    for (row_idx, row) in orchard.rows_iter().enumerate() {
        for (rev_col_idx, cell) in row.rev().scan(None, max_topography_indicator).enumerate() {
            let col_idx = orchard.columns - rev_col_idx - 1;
            tree_visibility[row_idx * orchard.columns + col_idx] |= cell;
        }
    }

    for (col_idx, col) in orchard.columns_iter().enumerate() {
        for (row_idx, cell) in col.scan(None, max_topography_indicator).enumerate() {
            tree_visibility[row_idx * orchard.columns + col_idx] |= cell
        }
    }

    for (col_idx, col) in orchard.columns_iter().enumerate() {
        for (rev_row_idx, cell) in col.rev().scan(None, max_topography_indicator).enumerate() {
            let row_idx = orchard.rows - rev_row_idx - 1;
            tree_visibility[row_idx * orchard.columns + col_idx] |= cell;
        }
    }

    println!("\nThe number of visibles trees is: {}", tree_visibility.iter().map(|v| match v {true => 1, false => 0}).sum::<u32>());

    //Part 2
    let mut scenic_scores = vec![0; orchard.rows * orchard.columns];
    for (row_idx, row) in orchard.rows_iter().enumerate() {
        for (col_idx, col) in row.enumerate() {
            let orientations = vec![OrchardIterDirection::Row, OrchardIterDirection::Column];
            let directions = vec![TreeNeighborsIterDirection::Forward, TreeNeighborsIterDirection::Backward];

            let score = orientations.iter()
                .flat_map(|o| 
                    directions.iter().map(|d| 
                        get_visible_tree_count(col, orchard.get_neighbors(row_idx, col_idx, *o, *d))))
                .product();
            scenic_scores[row_idx * orchard.columns + col_idx] = score;
        }
    }

    println!("The maximal scenic score is: {}", scenic_scores.iter().max().unwrap());
    
}

