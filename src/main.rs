#[cfg(test)]
mod tests;

pub struct Game {
    roll: usize,
    rolls: [u32;20]
}

impl Game {

    pub fn roll(&mut self, pins: u32) {
        self.rolls[self.roll] = pins;
        self.roll = self.roll + 1;
    }

    fn is_spare(&self, roll_index: usize) -> bool {
        self.rolls[roll_index] + self.rolls[roll_index+1] == 10
    }

    fn is_strike(&self, roll_index:usize) -> bool {
        self.rolls[roll_index] == 10
    }

    fn strike_bonus(&self, roll_index:usize) -> u32 {
        self.rolls[roll_index+1] + self.rolls[roll_index+2]
    }

    fn spare_bonus(&self, roll_index:usize) -> u32 {
        self.rolls[roll_index + 2]       
    }

    pub fn score(&self) -> u32 {
        let mut total = 0;
        let mut roll_index: usize = 0;
        for _frame_index in 0..10 {
            if self.is_strike(roll_index) {
                total = total + 10 + self.strike_bonus(roll_index);
                roll_index = roll_index + 1;
            }
            else if self.is_spare(roll_index) {
                total = total + 10 + self.spare_bonus(roll_index);
                roll_index = roll_index + 2;
            } else {
                total = total + self.rolls[roll_index] + self.rolls[roll_index + 1];
                roll_index = roll_index + 2;
            }
        }

        total
    }
}

impl Default for Game {
    fn default() -> Game {
        Game {
            roll: 0,
            rolls: <[u32;20]>::default()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
