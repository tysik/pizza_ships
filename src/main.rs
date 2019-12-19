mod arbiter;
mod ships;

use arbiter::*;
use ships::*;

// fn find_highest_score(current_streak: &mut ShipStreak, ship_streaks: &[ShipStreak], arbiter: &MainArbiter) -> u32 {
//     let next_streak_of_a_kind = match current_streak.kind {
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

fn main() {
    // INPUT
    let points_for_a = 1;
    let points_for_b = 2;
    let points_for_c = 3;
    let points_limit = 5;
    let ships_count = 12;
    let input_string = "AABBCCAABBCC";

    let arbiter = MainArbiter::make(points_for_a, points_for_b, points_for_c, points_limit);

    let ships = read_ships(ships_count, input_string);
    let ship_streaks = ShipStreak::from_ships(&ships);

    let ships_a = OneKindShipStreaks::make(ShipType::A, &ship_streaks);
    let ships_b = OneKindShipStreaks::make(ShipType::B, &ship_streaks);
    let ships_c = OneKindShipStreaks::make(ShipType::C, &ship_streaks);

    println!("Reading ships:");
    println!("{:?}", ships);
    println!("");

    for streak in ship_streaks.iter() {
        println!("{:?}", streak);
    }
    println!("");

    println!("{:?}", ships_a);
    println!("{:?}", ships_b);
    println!("{:?}", ships_c);
    println!("");

    for streak in ship_streaks.iter() {
        let value = arbiter.evaluate_streak(&streak);
        println!("value: {}", value);
    }
    println!("");
}
