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
    pub fn evaluate(&self) -> u32 {
        self.evaluate_swapped(0)
    }

    fn evaluate_continuated(&self, streak_idx: usize, continued_streak: &ShipStreak) -> u32 {
        let mut current_streak = if let Some(streak) = self.ship_streaks.get(streak_idx) {
            *streak
        } else {
            return self.arbiter.evaluate_streak(continued_streak);
        };

        current_streak.merge_with(continued_streak);

        // SWAP
        let swap_result =
            self.arbiter.evaluate_streak(&current_streak) + self.evaluate_swapped(streak_idx + 1);

        // CONTINUE
        let next_streak_idx = self.find_next_streak_idx(streak_idx, current_streak.kind);
        let cont_result = if let Some(idx) = next_streak_idx {
            self.evaluate_continuated(idx, &mut current_streak)
        } else {
            self.arbiter.evaluate_streak(&current_streak)
        };

        std::cmp::max(swap_result, cont_result)
    }

    fn evaluate_swapped(&self, streak_idx: usize) -> u32 {
        let mut current_streak = if let Some(streak) = self.ship_streaks.get(streak_idx) {
            streak
        } else {
            return 0;
        };

        // SWAP
        let swap_result =
            self.arbiter.evaluate_streak(current_streak) + self.evaluate_swapped(streak_idx + 1);

        // CONTINUE
        let next_streak_idx = self.find_next_streak_idx(streak_idx, current_streak.kind);
        let cont_result = if let Some(idx) = next_streak_idx {
            self.evaluate_continuated(idx, &mut current_streak)
        } else {
            self.arbiter.evaluate_streak(current_streak)
        };

        std::cmp::max(swap_result, cont_result)
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
        indices //.iter().rev().cloned().collect()
    }

    fn find_next_streak_idx(&self, current_idx: usize, kind: ShipType) -> Option<usize> {
        for idx in self.streak_indices[&kind].iter() {
            if *idx > current_idx {
                return Some(*idx);
            }
        }
        None
    }
}
