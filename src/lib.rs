#![allow(dead_code)]

mod day1;
mod day2;
mod day3;

fn read_input(day: &str) -> String {
    std::fs::read_to_string(format!("input/{}.txt", day)).unwrap()
}
