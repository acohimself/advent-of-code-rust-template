pub mod days;

use std::fs;

/// Read input file for a given day
pub fn read_input(day: u8) -> String {
    let path = format!("input/day{:02}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
}

/// Read input file and return lines as a vector
pub fn read_lines(day: u8) -> Vec<String> {
    read_input(day).lines().map(|s| s.to_string()).collect()
}

/// Parse input into a vector of numbers
pub fn parse_numbers<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// Split input by blank lines into groups
pub fn split_by_blank_lines(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_string()).collect()
}
