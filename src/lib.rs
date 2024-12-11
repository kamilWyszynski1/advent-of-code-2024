#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn read_input(day: &str) -> String {
    std::fs::read_to_string(format!("input/{}.txt", day)).unwrap()
}
