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

impl ShipStreak {
    fn from_ships(ships: &Vec<ShipType>) -> Vec<Self> {
        assert!(ships.len() > 0, "trying to create streaks from empty ship vector");

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
}

#[derive(Debug)]
struct OneKindShipStreaks {
    indices: Vec<u32>,
    kind: ShipType,
}

impl OneKindShipStreaks {
    fn make(kind: ShipType, ship_streaks: &Vec<ShipStreak>) -> Self {
        let mut indices = Vec::new();
        for streak in ship_streaks {
            if streak.kind == kind {
                indices.push(streak.start_idx);
            }
        }

        Self {
            indices,
            kind,
        }
    }
}

#[derive(Debug)]
struct AuxArbiter {
    points_for_first_ship: u32,
    ships_before_limit: u32,
    points_before_limit: u32,
    points_limit: u32,
}

impl AuxArbiter {
    fn make(points_for_first_ship: u32, points_limit: u32) -> Self {
        let (ships_before_limit, points_before_limit) =
            AuxArbiter::count_ship_limits(points_for_first_ship, points_limit);

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

    fn evaluate_points(&self, ships_count: u32) -> u32 {
        if ships_count > self.ships_before_limit {
            return self.points_before_limit
                + (ships_count - self.ships_before_limit) * self.points_limit;
        } else {
            // Arithmetic series
            return self.points_for_first_ship * ships_count * (ships_count + 1) / 2;
        }
    }
}

#[derive(Debug)]
struct MainArbiter {
    arbiter_a: AuxArbiter,
    arbiter_b: AuxArbiter,
    arbiter_c: AuxArbiter,
}

impl MainArbiter {
    fn make(points_for_a: u32, points_for_b: u32, points_for_c: u32, points_limit: u32) -> Self {
        let arbiter_a = AuxArbiter::make(points_for_a, points_limit);
        let arbiter_b = AuxArbiter::make(points_for_b, points_limit);
        let arbiter_c = AuxArbiter::make(points_for_c, points_limit);

        Self {
            arbiter_a,
            arbiter_b,
            arbiter_c,
        }
    }

    fn evaluate_streak(&self, ship_streak: &ShipStreak) -> u32 {
        match ship_streak.kind {
            ShipType::A => self.arbiter_a.evaluate_points(ship_streak.count),
            ShipType::B => self.arbiter_b.evaluate_points(ship_streak.count),
            ShipType::C => self.arbiter_c.evaluate_points(ship_streak.count),
        }
    }
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

fn main() {
    // INPUT
    let points_for_a = 1;
    let points_for_b = 2;
    let points_for_c = 3;
    let points_limit = 5;
    let ships_count = 10;
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
