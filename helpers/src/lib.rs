use std::{fs::File, io::Read};
use anyhow::Result;

pub fn read_input_file(day: &str,input_type: &str) -> Result<String> {
    let mut file = File::open(format!("{day}/data/{input_type}.txt"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub trait SquareText {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get(&self, x: usize, y: usize) -> u8;
    
    // Add convenience methods for different integer types
    fn width_i32(&self) -> i32 {
        self.get_width() as i32
    }
    
    fn height_i32(&self) -> i32 {
        self.get_height() as i32
    }

    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width_i32() && y < self.height_i32()
    }

    fn is_outside(&self, x: i32, y: i32) -> bool {
        !self.is_inside(x, y)
    }

    fn get_i32(&self, x: i32, y: i32) -> u8 {
        self.get(x as usize, y as usize)
    }
}

impl SquareText for &[Vec<u8>] {
    fn get_width(&self) -> usize {
        self[0].len()
    }

    fn get_height(&self) -> usize {
        self.len()
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self[y][x]
    }
}

impl SquareText for Vec<Vec<u8>> {
    fn get_width(&self) -> usize {
        self[0].len()
    }

    fn get_height(&self) -> usize {
        self.len()
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self[y][x]
    }
}

pub const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
