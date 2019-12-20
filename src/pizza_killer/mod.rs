mod arbiter;
mod ships;

use arbiter::*;
use ships::*;
use std::collections::HashMap;

pub struct PizzaKiller {
    arbiter: Arbiter,
    ship_streaks: Vec<ShipStreak>,
    streak_indices: HashMap<ShipType, Vec<usize>>,
}

impl PizzaKiller {
    pub fn make(
        points_for_a: u32,
        points_for_b: u32,
        points_for_c: u32,
        points_limit: u32,
        ships_count: u32,
        input_string: &str,
    ) -> Self {
        let arbiter = Arbiter::make(points_for_a, points_for_b, points_for_c, points_limit);
        let ships = read_ships(ships_count, input_string);
        let ship_streaks = ShipStreak::from_ships(&ships);
        let mut streak_indices = HashMap::new();
        streak_indices.insert(
            ShipType::A,
            Self::get_streak_indices(ShipType::A, &ship_streaks),
        );
        streak_indices.insert(
            ShipType::B,
            Self::get_streak_indices(ShipType::B, &ship_streaks),
        );
        streak_indices.insert(
            ShipType::C,
            Self::get_streak_indices(ShipType::C, &ship_streaks),
        );

        Self {
            arbiter,
            ship_streaks,
            streak_indices,
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.arbiter);
        println!("");

        for streak in self.ship_streaks.iter() {
            println!("{:?}", streak);
        }
        println!("");

        for (key, ref value) in self.streak_indices.iter() {
            println!("Type {:?}: {:?}", key, value);
        }
        println!("");

        for streak in self.ship_streaks.iter() {
            let value = self.arbiter.evaluate_streak(&streak);
            println!("value: {}", value);
        }
        println!("");
    }

    fn get_streak_indices(kind: ShipType, ship_streaks: &Vec<ShipStreak>) -> Vec<usize> {
        let mut indices = Vec::new();
        for (idx, streak) in ship_streaks.iter().enumerate() {
            if streak.kind == kind {
                indices.push(idx);
            }
        }
        indices.iter().rev().cloned().collect()
    }

    fn get_best_score(&mut self) -> u32 {
        self.evaluate(0, None)
    }

    // Evaluate best score starting with streak_idx
    fn evaluate(&mut self, streak_idx: usize, continued_streak: Option<&mut ShipStreak>) -> u32 {
        // Continue means to look for another streak of the same type
        // Swap means to switch to another type
        
        // 1. Check if the current streak_idx is pointing to a valid object
        // If yes - get the object and move on
        // If no - check if there was continued streak and evaluate it or return 0 (end of streaks)
        let mut current_streak = if let Some(streak) = self.ship_streaks.get(streak_idx) {
            streak
        } else if let Some(prev_streak) = continued_streak {
            return self.arbiter.evaluate_streak(prev_streak);
        } else {
            return 0;
        };

        // Find if there is any index for continuing with the same type of ships
        let continue_streak_idx = self
            .streak_indices
            .get_mut(&current_streak.kind)
            .unwrap()
            .pop();

        // If continuation index was found, check if there was already continuated streak
        // If there was 
        let continue_result = if let Some(idx) = continue_streak_idx {
            if let Some(prev_streak) = continued_streak {
                prev_streak.merge_with(&self.ship_streaks[idx]);
                self.evaluate(idx, Some(prev_streak))
            } else {
                self.evaluate(idx, Some(&mut current_streak))
            }            
        } else {
            self.arbiter.evaluate_streak()
        };



        if let Some(prev_streak) = continued_streak {}

        let swap_streak_idx = streak_idx + 1;
        let mut swap_result = self.arbiter.evaluate_streak(continued_streak);
        if let Some(streak) = self.ship_streaks.get(swap_streak_idx) {
            swap_result += self.evaluate(swap_streak_idx, None);
        }

        std::cmp::max(continue_result, swap_result)
    }
}

// fn find_highest_score(continued_streak: &mut ShipStreak, ship_streaks: &[ShipStreak], arbiter: &Arbiter) -> u32 {
//     let next_streak_of_a_kind = match continued_streak.kind {
//         ShipType::A =>
//     }
//     let current_value = arbiter.evaluate_streak(&ship_streaks[0]);

//     let value_after_switch = arbiter.evaluate_streak(&ship_streak[1..]);
//     // find index of continuation

//     let value_after_continue = arbiter.evaluate_streak(&ship_streak[1..]);
//     // find points for this value

//     // check values for points after swap (starts from zero)
//     // check values for points after continue (needs to know type and current streak)
//     // choose better value.
// }
