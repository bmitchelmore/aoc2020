use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Day;

#[derive(Clone)]
pub struct SeatLayout {
    cells: Vec<Vec<Cell>>
}

impl SeatLayout {
    fn iterate(&mut self) -> bool {
        let mut next = self.cells.clone();
        let mut changed = false;
        let last_index = self.cells.len() - 1;
        for row in 0..=last_index {
            let last_index = self.cells[row].len() - 1;
            for col in 0..=last_index {
                let occupied_count = self.occupied_count_near(col, row);
                if self.is_empty_at(col, row) && occupied_count == 0 {
                    next[row][col] = Cell::Occupied;
                    changed = true;
                }
                if self.is_occupied_at(col, row) && occupied_count >= 4 {
                    next[row][col] = Cell::Empty;
                    changed = true;
                }
            }
        }
        if changed {
            self.cells = next;
        }
        changed
    }
    fn occupied_cells_count(&self) -> u64 {
        let mut count = 0;
        let last_index = self.cells.len() - 1;
        for row in 0..=last_index {
            let last_index = self.cells[row].len() - 1;
            for col in 0..=last_index {
                if self.is_occupied_at(col, row) {
                    count += 1;
                }
            }
        }
        count
    }
    fn cell_at(&self, x: usize, y: usize) -> Option<&Cell> {
        if let Some(row) = self.cells.get(y) {
            if let Some(cell) = row.get(x) {
                return Some(cell);
            }
        }
        None
    }
    fn is_occupied_at(&self, x: usize, y: usize) -> bool {
        if let Some(cell) = self.cell_at(x, y) {
            return *cell == Cell::Occupied;
        }
        false
    }
    fn is_empty_at(&self, x: usize, y: usize) -> bool {
        if let Some(cell) = self.cell_at(x, y) {
            return *cell == Cell::Empty;
        }
        false
    }
    fn occupied_count_near(&self, x: usize, y: usize) -> u32 {
        let mut occupied = 0;
        if y > 0 { 
            if let Some(top_row) = self.cells.get(y - 1) {
                if x > 0 {
                    if let Some(top_left_cell) = top_row.get(x - 1) {
                        if *top_left_cell == Cell::Occupied {
                            occupied += 1;
                        }
                    }
                }
                if let Some(top_middle_cell) = top_row.get(x) {
                    if *top_middle_cell == Cell::Occupied {
                        occupied += 1;
                    }
                }
                if x < usize::MAX {
                    if let Some(top_right_cell) = top_row.get(x + 1) {
                        if *top_right_cell == Cell::Occupied {
                            occupied += 1;
                        }
                    }
                }
            }
        }
        if let Some(current_row) = self.cells.get(y) {
            if x > 0 {
                if let Some(left_cell) = current_row.get(x - 1) {
                    if *left_cell == Cell::Occupied {
                        occupied += 1;
                    }
                }
            }
            if x < usize::MAX {
                if let Some(right_cell) = current_row.get(x + 1) {
                    if *right_cell == Cell::Occupied {
                        occupied += 1;
                    }
                }
            }
        }
        if y < usize::MAX { 
            if let Some(bottom_row) = self.cells.get(y + 1) {
                if x > 0 {
                    if let Some(bottom_left_cell) = bottom_row.get(x - 1) {
                        if *bottom_left_cell == Cell::Occupied {
                            occupied += 1;
                        }
                    }
                }
                if let Some(bottom_middle_cell) = bottom_row.get(x) {
                    if *bottom_middle_cell == Cell::Occupied {
                        occupied += 1;
                    }
                }
                if x < usize::MAX {
                    if let Some(bottom_right_cell) = bottom_row.get(x + 1) {
                        if *bottom_right_cell == Cell::Occupied {
                            occupied += 1;
                        }
                    }
                }
            }
        }
        occupied
    }
    
    fn iterate_part2(&mut self) -> bool {
        let mut next = self.cells.clone();
        let mut changed = false;
        let last_index = self.cells.len() - 1;
        for row in 0..=last_index {
            let last_index = self.cells[row].len() - 1;
            for col in 0..=last_index {
                let occupied_count = self.occupied_count_near_part2(col, row);
                if self.is_empty_at(col, row) && occupied_count == 0 {
                    next[row][col] = Cell::Occupied;
                    changed = true;
                }
                if self.is_occupied_at(col, row) && occupied_count >= 5 {
                    next[row][col] = Cell::Empty;
                    changed = true;
                }
            }
        }
        if changed {
            self.cells = next;
        }
        changed
    }
    fn first_seat_in(&self, current_x: usize, current_y: usize, x_dir: i32, y_dir: i32) -> Option<&Cell> {
        let current_x = current_x as i32;
        let current_y = current_y as i32;
        let max_x = (self.cells[0].len() - 1) as i32;
        let max_y = (self.cells.len() - 1) as i32;
        let min_x = 0;
        let min_y = 0;
        let mut target_x = current_x + x_dir;
        let mut target_y = current_y + y_dir;
        while target_x >= min_x && target_x <= max_x && target_y >= min_y && target_y <= max_y {
            if let Some(cell) = self.cell_at(target_x as usize, target_y as usize) {
                if *cell == Cell::Occupied || *cell == Cell::Empty {
                    return Some(cell);
                }
            }
            target_x += x_dir;
            target_y += y_dir;
        }
        None
    }
    fn occupied_count_near_part2(&self, x: usize, y: usize) -> u32 {
        let mut occupied = 0;
        if let Some(top_left_cell) = self.first_seat_in(x, y, -1, -1) {
            if *top_left_cell == Cell::Occupied {
                occupied += 1;
            }
        }
        if let Some(top_middle_cell) = self.first_seat_in(x, y, 0, -1) {
            if *top_middle_cell == Cell::Occupied {
                occupied += 1;
            }
        }
        if let Some(top_right_cell) = self.first_seat_in(x, y, 1, -1) {
            if *top_right_cell == Cell::Occupied {
                occupied += 1;
            }
        }
        if let Some(left_cell) = self.first_seat_in(x, y, -1, 0) {
            if *left_cell == Cell::Occupied {
                occupied += 1;
            }
        }
        if let Some(right_cell) = self.first_seat_in(x, y, 1, 0) {
            if *right_cell == Cell::Occupied {
                occupied += 1;
            }
        }
        if let Some(bottom_left_cell) = self.first_seat_in(x, y, -1, 1) {
            if *bottom_left_cell == Cell::Occupied {
                occupied += 1;
            }
        }
        if let Some(bottom_middle_cell) = self.first_seat_in(x, y, 0, 1) {
            if *bottom_middle_cell == Cell::Occupied {
                occupied += 1;
            }
        }
        if let Some(bottom_right_cell) = self.first_seat_in(x, y, 1, 1) {
            if *bottom_right_cell == Cell::Occupied {
                occupied += 1;
            }
        }
        occupied
    }
}

impl fmt::Display for SeatLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub enum Cell {
    Floor,
    Empty,
    Occupied
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Cell::Empty, Cell::Empty) | (Cell::Occupied, Cell::Occupied) | (Cell::Floor, Cell::Floor) => true,
            _ => false
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cell::Floor => '.',
            Cell::Empty => 'L',
            Cell::Occupied => '#'
        };
        write!(f, "{}", c)
    }
}

pub struct Day11 {}

impl Day for Day11 { 
    type Input = SeatLayout;
    type Output = u64;

    fn read() -> SeatLayout {
        let mut data: Vec<Vec<Cell>> = vec![];
        let file = File::open("./src/day11/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let mut row = vec![];
            for c in line.trim().chars() {
                let cell = match c {
                    'L' => Cell::Empty,
                    '#' => Cell::Occupied,
                    '.' => Cell::Floor,
                    _ => panic!("Unexpected cell value: {:?}", c)
                };
                row.push(cell);
            }
            data.push(row);
        }
        SeatLayout { cells: data }
    }

    fn part1(input: &SeatLayout) -> u64 {
        let mut layout = input.clone();
        while layout.iterate() {}
        layout.occupied_cells_count()
    }

    fn part2(input: &SeatLayout) -> u64 {
        let mut layout = input.clone();
        while layout.iterate_part2() {}
        layout.occupied_cells_count()
    }
}