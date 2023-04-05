use super::tile_checker::TileChecker;

pub struct TileSolver {
    pub tile_layout: [i32; 25],
    working_copy: [i32; 25],
    solution: [i32; 25],
}

impl TileSolver {
    // TODO: Static method that generates a random puzzle
    pub fn generate_random_puzzle() -> Self {
        let mut layout = [0; 25];
        let mut tile_checker = TileChecker::new();
        for i in 0..25 {
            layout[i] = match tile_checker.check(rand::random()) {
                true => 1,
                false => 0,
            };
        }
        Self::new(layout)
    }

    pub fn new(layout: [i32; 25]) -> Self {
        Self {
            tile_layout: layout.clone(),
            working_copy: layout.clone(),
            solution: [0; 25],
        }
    }
    // Returns the number of moves it'll take to solve the puzzle using the chase the light algorithm, along with the minimum number of moves
    pub fn solve(&mut self) -> (i32, i32) {
        self.working_copy = self.tile_layout.clone();
        let mut actions = self.chase_the_light();

        if !self.is_solved() {
          actions += self.check_bottom_row();
          actions += self.chase_the_light();
        }
        
        let minimum_actions = self.solution.iter().sum();

        (actions, minimum_actions)
    }

    // Technique to solve the puzzle
    fn chase_the_light(&mut self) -> i32 {
      let mut actions = 0;
      for y in 1..5 {
        for x in 0..5 {
          let index = (y * 5) + x;
          let above = self.working_copy[index - 5];
          if self.working_copy[above as usize] == 1 {
            self.toggle_with_neighbors(x as i32, y as i32);
            actions+=1;
          }
        }
      }
      actions
    }
    // Based on the bottom row, press a few buttons and continue
    fn check_bottom_row(&mut self) -> i32 {
      let mut actions = 0;
      if self.working_copy[20] == 1 {     // If A5 is on, press D1 and E1
        self.toggle_with_neighbors(0, 3);
        actions+=1;
        self.toggle_with_neighbors(0, 4);
        actions+=1;
      }
      if self.working_copy[22] == 1 {     // If B5 is on, press B1 and E1
        self.toggle_with_neighbors(0, 1);
        actions+=1;
        self.toggle_with_neighbors(0, 4);
        actions+=1;
      }
      if self.working_copy[23] == 1 {     // If C1 is on, press D1
        self.toggle_with_neighbors(0, 3);
        actions+=1;
      }
      actions
    }

    fn toggle_with_neighbors(&mut self, x: i32, y: i32){
      
        self.toggle(x, y);
        self.toggle(x - 1, y);
        self.toggle(x + 1, y);
        self.toggle(x, y - 1);
        self.toggle(x, y + 1);

        // Mark the solution
        let index = (y * 5) + x;
        self.solution[index as usize] = match self.solution[index as usize] {
          0 => 1,
          _ => 0,
        };
    }
    fn toggle(&mut self, x: i32, y: i32) {
        if x < 0 || x > 4 || y < 0 || y > 4 {
            return;
        }
        let index = (y * 5) + x;
        self.working_copy[index as usize] = match self.working_copy[index as usize] {
            0 => 1,
            _ => 0,
        };
    }

    fn is_solved(&mut self) -> bool {
        for i in 0..25 {
            if self.working_copy[i] == 0 {
                return false;
            }
        }
        true
    }
}
