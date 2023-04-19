use bevy::prelude::{Resource, info};

use super::puzzle_word_encoder;

const PATTERN_1: [i32; 25] = [
    1, 0, 1, 0, 1, 
    1, 0, 1, 0, 1, 
    0, 0, 0, 0, 0, 
    1, 0, 1, 0, 1, 
    1, 0, 1, 0, 1,
];
const PATTERN_2: [i32; 25] = [
    1, 1, 0, 1, 1, 
    0, 0, 0, 0, 0, 
    1, 1, 0, 1, 1, 
    0, 0, 0, 0, 0, 
    1, 1, 0, 1, 1,
];

#[derive(Default, Resource)]
pub struct TilePuzzle {
    pub width: i32,
    pub height: i32,
    pub tile_values: Vec<bool>,
    pub puzzle_short_code: String,
}

impl TilePuzzle {
    pub fn new() -> TilePuzzle {
        TilePuzzle {
            width:5,
            height:5,
            tile_values: vec![false; 25 as usize],
            puzzle_short_code: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn generate_random_puzzle(&mut self) {
        let mut pattern_1_matches = 0;
        let mut pattern_2_matches = 0;
        
        for index in 0..self.tile_values.len() {
            let mut on = rand::random();
            // Index 23, 
            //if both pattern counts are even, leave them alone
            // if pattern 1 is even and pattern 2 is odd, turn it on
            // if pattern 2 is odd and pattern 1 is odd, leave it off
            // if pattern 2 is odd and pattern 1 is even, turn it on
            if index == 23 {
                on = (pattern_1_matches % 2 == 0) ^ (pattern_2_matches % 2 == 0)
            } 
            // If pattern 1 is odd, turn it on
            else if index == 24 {
                on = pattern_1_matches % 2 == 1;
            }
            self.tile_values[index] = on;
            pattern_1_matches += match on { true => PATTERN_1[index], false => 0 };
            pattern_2_matches += match on { true => PATTERN_2[index], false => 0 };
        }

        self.encode_puzzle();
        info!("Puzzle: {}", self.puzzle_short_code);
    }

    pub fn encode_puzzle(&mut self) {
        self.puzzle_short_code = puzzle_word_encoder::encode_level_as_words(&self.tile_values);
    }

    #[allow(dead_code)]
    pub fn easy_puzzle(&mut self) {
        for index in 0..self.tile_values.len() {
            self.tile_values[index] = false;
        }
        self.tile_values[0] = true;
        self.tile_values[1] = true;
        self.tile_values[5] = true;
        // 24
        // 16
        // 0
        // 0
        // 0
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