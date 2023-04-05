// Valid puzzles have an even number of positions that
// 1. have an even number of positions matching pattern1 turned on
// 2. have an even number of positions matching pattern2 turned on

const PATTERN_1: [i32; 25] = [
    0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0,
];
const PATTERN_2: [i32; 25] = [
    0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0,
];

pub struct TileChecker {
    pattern_1_matches: i32,
    pattern_2_matches: i32,
    index: i32,
}
impl TileChecker {
    pub fn new() -> Self {
        Self {
            pattern_1_matches: 0,
            pattern_2_matches: 0,
            index: 0,
        }
    }
    pub fn check(&mut self, on: bool) -> bool {
        let mut final_on = on;
        let pattern_1_match = PATTERN_1[self.index as usize] == 0;
        let pattern_2_match = PATTERN_2[self.index as usize] == 0;

        // For the last two, we need to ensure that a valid pattern is created
        if self.index == 23 {
            final_on = (self.pattern_1_matches % 2 == 0) ^ (self.pattern_2_matches % 2 == 0)
        }
        if self.index == 24 {
            final_on = self.pattern_2_matches % 2 == 1;
        }

        self.pattern_1_matches += match pattern_1_match && final_on {
            true => 1,
            false => 0,
        };

        self.pattern_2_matches += match pattern_2_match && final_on {
            true => 1,
            false => 0,
        };

        self.index += 1;
        final_on
    }
}
