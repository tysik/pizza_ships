mod pizza_killer;
use pizza_killer::*;

fn main() {
    // INPUT
    let points_for_a = 1;
    let points_for_b = 2;
    let points_for_c = 3;
    let points_limit = 5;
    let ships_count = 12;
    let input_string = "AABBCCAABBCC";

    let pizza = PizzaKiller::make(
        points_for_a,
        points_for_b,
        points_for_c,
        points_limit,
        ships_count,
        input_string,
    );

    pizza.print();
}
