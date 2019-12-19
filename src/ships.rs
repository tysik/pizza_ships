#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ShipType {
    A,
    B,
    C,
}

#[derive(Debug, Copy, Clone)]
pub struct ShipStreak {
    pub kind: ShipType,
    pub count: u32,
    start_idx: u32,    
}

#[derive(Debug)]
pub struct OneKindShipStreaks {
    kind: ShipType,
    indices: Vec<u32>,
}

impl ShipStreak {
    pub fn from_ships(ships: &Vec<ShipType>) -> Vec<Self> {
        assert!(
            ships.len() > 0,
            "trying to create streaks from empty ship vector"
        );

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

impl OneKindShipStreaks {
    pub fn make(kind: ShipType, ship_streaks: &Vec<ShipStreak>) -> Self {
        let mut indices = Vec::new();
        for streak in ship_streaks {
            if streak.kind == kind {
                indices.push(streak.start_idx);
            }
        }

        Self { indices, kind }
    }
}

pub fn read_ships(ships_count: u32, input_string: &str) -> Vec<ShipType> {
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
