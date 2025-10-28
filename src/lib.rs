pub mod days;

use std::fs;

/// Read input file for a given day and return as a single string
pub fn read_input(day: u8) -> String {
    let path = format!("input/day{:02}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
}

/// Parse a string into a vector of numbers, one per line
///
/// # Example
/// ```
/// let input = "1\n2\n3";
/// let numbers: Vec<i32> = parse_numbers(input);
/// assert_eq!(numbers, vec![1, 2, 3]);
/// ```
pub fn parse_numbers<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// Split a string by blank lines (double newlines) into groups
///
/// Useful for puzzles where input is separated into groups
pub fn split_by_blank_lines(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_string()).collect()
}
