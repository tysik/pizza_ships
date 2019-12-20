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
        self.eval(0, None)
    }

    fn eval(&self, streak_idx: usize, continued_streak: Option<&ShipStreak>) -> u32 {
        let mut current_streak = if let Some(streak) = self.ship_streaks.get(streak_idx) {
            *streak
        } else if let Some(prev_streak) = continued_streak {
            return self.arbiter.evaluate_streak(prev_streak);
        } else {
            return 0;
        };

        if let Some(prev_streak) = continued_streak {
            current_streak.merge_with(prev_streak);
        }

        // SWAP
        let swap_result =
            self.arbiter.evaluate_streak(&current_streak) + self.eval(streak_idx + 1, None);

        // CONTINUE
        let next_streak_idx = self.find_next_streak_idx(streak_idx, current_streak.kind);
        let cont_result = if let Some(idx) = next_streak_idx {
            self.eval(idx, Some(&current_streak))
        } else {
            self.arbiter.evaluate_streak(&current_streak)
        };

        std::cmp::max(swap_result, cont_result)
    }

    fn get_streak_indices(kind: ShipType, ship_streaks: &Vec<ShipStreak>) -> Vec<usize> {
        let mut indices = Vec::new();
        for (idx, streak) in ship_streaks.iter().enumerate() {
            if streak.kind == kind {
                indices.push(idx);
            }
        }
        indices
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
