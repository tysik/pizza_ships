mod pizza_killer;
use pizza_killer::*;

use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

#[derive(Debug)]
struct TestInput {
    ships_count: u32,
    points_limit: u32,
    points_a: u32,
    points_b: u32,
    points_c: u32,
    ships: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Bad number of arguments");

    let path = Path::new(&args[1]);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut tests_count = String::new();
    reader.read_line(&mut tests_count).unwrap();
    let tests_count: usize = tests_count.trim().to_owned().parse().unwrap();

    println!("Test cases: {}", tests_count);

    let mut tests: Vec<TestInput> = Vec::new();
    tests.reserve(tests_count);

    for _ in 0..tests_count {
        let mut n_and_m = String::new();
        reader.read_line(&mut n_and_m).unwrap();
        let v1: Vec<&str> = n_and_m.split(' ').collect();
        assert!(v1.len() == 2, "Bad first line");

        let ships_count: u32 = v1[0].trim().to_owned().parse().unwrap();
        let points_limit: u32 = v1[1].trim().to_owned().parse().unwrap();

        let mut points = String::new();
        reader.read_line(&mut points).unwrap();
        let v2: Vec<&str> = points.split(' ').collect();
        assert!(v2.len() == 3, "Bad second line");

        let points_a: u32 = v2[0].trim().to_owned().parse().unwrap();
        let points_b: u32 = v2[1].trim().to_owned().parse().unwrap();
        let points_c: u32 = v2[2].trim().to_owned().parse().unwrap();

        let mut ships = String::new();
        reader.read_line(&mut ships).unwrap();
        ships = ships.trim().to_owned();

        let test_input = TestInput {
            ships_count,
            points_limit,
            points_a,
            points_b,
            points_c,
            ships,
        };
        tests.push(test_input);
    }

    for test in tests {
        println!("{:?}", test);
        let pizza = PizzaKiller::make(
            test.points_a,
            test.points_b,
            test.points_c,
            test.points_limit,
            test.ships_count,
            &test.ships,
        );

        let result = pizza.evaluate();
        println!("Result: {}", result);
    }
}
