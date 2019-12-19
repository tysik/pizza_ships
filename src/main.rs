#[derive(Debug, Copy, Clone, PartialEq)]
enum ShipType {
    A,
    B,
    C,
}

#[derive(Debug, Copy, Clone)]
struct ShipStreak {
    start_idx: u32,
    count: u32,
    kind: ShipType,
}

#[derive(Debug)]
struct Arbiter {
    points_a: u32,
    points_b: u32,
    points_c: u32,
    points_max: u32,

    ships_a_before_limit: u32,
    ships_b_before_limit: u32,
    ships_c_before_limit: u32,

    points_a_before_limit: u32,
    points_b_before_limit: u32,
    points_c_before_limit: u32,
}

impl Arbiter {
    fn make(points_a: u32, points_b: u32, points_c: u32, points_max: u32) -> Self {
        let (ships_a_before_limit, points_a_before_limit) =
            ships_before_limit(points_a, points_max);
        let (ships_b_before_limit, points_b_before_limit) =
            ships_before_limit(points_b, points_max);
        let (ships_c_before_limit, points_c_before_limit) =
            ships_before_limit(points_c, points_max);

        Self {
            points_a,
            points_b,
            points_c,
            points_max,
            ships_a_before_limit,
            ships_b_before_limit,
            ships_c_before_limit,
            points_a_before_limit,
            points_b_before_limit,
            points_c_before_limit,
        }
    }

    fn evaluate_streak(&self, ship_streak: &ShipStreak) -> u32 {
        if ship_streak.kind == ShipType::A {
            if ship_streak.count > self.ships_a_before_limit {
                return self.points_a_before_limit
                    + (ship_streak.count - self.ships_a_before_limit) * self.points_max;
            } else {
                return self.points_a * ship_streak.count * (ship_streak.count + 1) / 2
            }
        }
        9999999
    }
}

fn ships_before_limit(points_per_ship: u32, limit: u32) -> (u32, u32) {
    let mut ships_count = 0;
    loop {
        let points = (ships_count + 1) * points_per_ship;
        if points >= limit {
            break;
        }
        ships_count += 1;
    }
    // Arithmetic series
    let total_points = points_per_ship * ships_count * (ships_count + 1) / 2;
    (ships_count, total_points)
}

fn read_ships(ships_count: u32, input_string: &str) -> Vec<ShipType> {
    let mut ships: Vec<ShipType> = Vec::new();
    ships.reserve(ships_count as usize);

    for letter in input_string.chars() {
        let ship = match letter {
            'A' => ShipType::A,
            'B' => ShipType::B,
            'C' => ShipType::C,
            _ => panic!(),
        };
        ships.push(ship);
    }
    ships
}

fn turn_into_streaks(ships: &Vec<ShipType>) -> Vec<ShipStreak> {
    let mut ship_streaks = Vec::new();

    let mut current_streak = ShipStreak {
        kind: ships[0],
        count: 0,
        start_idx: 0,
    };

    for (idx, kind) in ships.iter().enumerate() {
        if *kind == current_streak.kind {
            current_streak.count += 1;
        } else {
            ship_streaks.push(current_streak.clone());
            current_streak.kind = *kind;
            current_streak.count = 1;
            current_streak.start_idx = idx as u32;
        }
    }
    ship_streaks.push(current_streak);
    ship_streaks
}

fn main() {
    // INPUT
    let points_a = 1;
    let points_b = 2;
    let points_c = 3;
    let points_max = 5;
    let ships_count = 10;
    let input_string = "AABBCCAABBCC";

    let arbiter = Arbiter::make(points_a, points_b, points_c, points_max);
    println!("{:?}", arbiter);

    let ships = read_ships(ships_count, input_string);

    for ship in ships.iter() {
        print!("{:?}", ship);
    }
    println!("");

    let ship_streaks = turn_into_streaks(&ships);
    for streak in ship_streaks.iter() {
        println!("{:?}", streak);
        let value = arbiter.evaluate_streak(&streak);
        println!("value: {}", value);
    }
    println!("");
}
