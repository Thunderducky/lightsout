use bevy::prelude::Resource;

const PATTERN_1: [i32; 25] = [
    0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0,
];
const PATTERN_2: [i32; 25] = [
    0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0,
];

#[derive(Default, Resource)]
pub struct TilePuzzle {
    pub width: i32,
    pub height: i32,
    pub tile_values: Vec<bool>
}

impl TilePuzzle {
    pub fn new() -> TilePuzzle {
        TilePuzzle {
            width:5,
            height:5,
            tile_values: vec![false; 25 as usize]
        }
    }

    pub fn generate_random_puzzle(&mut self) {
        let mut pattern_1_matches = 0;
        let mut pattern_2_matches = 0;
        for index in 0..self.tile_values.len() {
            let mut on = rand::random();
            if index == 23 {
                on = (pattern_1_matches % 2 == 0) ^ (pattern_2_matches % 2 == 0)
            } else if index == 24 {
                on = pattern_2_matches % 2 == 1;
            }
            self.tile_values[index] = on;
            pattern_1_matches += PATTERN_1[index];
            pattern_2_matches += PATTERN_2[index];
        }
    }

    pub fn easy_puzzle(&mut self) {
        for index in 0..self.tile_values.len() {
            self.tile_values[index] = false;
        }
        self.tile_values[0] = true;
        self.tile_values[1] = true;
        self.tile_values[5] = true;
    }

    pub fn toggle_tile(&mut self, x: i32, y: i32) {
        let index = (y * self.width) + x;
        self.tile_values[index as usize] = !self.tile_values[index as usize];
        self.toggle(x - 1, y);
        self.toggle(x + 1, y);
        self.toggle(x, y - 1);
        self.toggle(x, y + 1);
    }

    fn toggle(&mut self, x: i32, y: i32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        let index = (y * self.width) + x;
        self.tile_values[index as usize] = !self.tile_values[index as usize];
    }

    pub fn is_solved(&mut self) -> bool {
        for index in 0..self.tile_values.len() {
            if self.tile_values[index] {
                return false;
            }
        }
        true
    }
}