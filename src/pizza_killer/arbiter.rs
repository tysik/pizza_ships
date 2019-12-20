use super::ships::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Arbiter {
    sub_arbiters: HashMap<ShipType, SubArbiter>,
}

#[derive(Debug)]
pub struct SubArbiter {
    points_for_first_ship: u32,
    ships_before_limit: u32,
    points_before_limit: u32,
    points_limit: u32,
}

impl Arbiter {
    pub fn make(
        points_for_a: u32,
        points_for_b: u32,
        points_for_c: u32,
        points_limit: u32,
    ) -> Self {
        let mut sub_arbiters = HashMap::new();
        sub_arbiters.insert(ShipType::A, SubArbiter::make(points_for_a, points_limit));
        sub_arbiters.insert(ShipType::B, SubArbiter::make(points_for_b, points_limit));
        sub_arbiters.insert(ShipType::C, SubArbiter::make(points_for_c, points_limit));

        Self { sub_arbiters }
    }

    pub fn evaluate_streak(&self, ship_streak: &ShipStreak) -> u32 {
        self.sub_arbiters[&ship_streak.kind].evaluate_points(ship_streak.count)
    }
}

impl SubArbiter {
    pub fn evaluate_points(&self, ships_count: u32) -> u32 {
        if ships_count > self.ships_before_limit {
            return self.points_before_limit
                + (ships_count - self.ships_before_limit) * self.points_limit;
        } else {
            // Use sum of arithmetic series
            return self.points_for_first_ship * ships_count * (ships_count + 1) / 2;
        }
    }

    fn make(points_for_first_ship: u32, points_limit: u32) -> Self {
        let (ships_before_limit, points_before_limit) =
            Self::count_ship_limits(points_for_first_ship, points_limit);

        Self {
            points_for_first_ship,
            ships_before_limit,
            points_before_limit,
            points_limit,
        }
    }

    fn count_ship_limits(points_for_first_ship: u32, points_limit: u32) -> (u32, u32) {
        let mut ships_count = 0;
        loop {
            let points = (ships_count + 1) * points_for_first_ship;
            if points >= points_limit {
                break;
            }
            ships_count += 1;
        }
        // Arithmetic series
        let total_points = points_for_first_ship * ships_count * (ships_count + 1) / 2;
        (ships_count, total_points)
    }
}
