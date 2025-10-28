# Advent of Code - Rust Template

A clean, efficient template for solving Advent of Code puzzles in Rust.

## Features

- Single binary with day selection via CLI
- Common utility functions for parsing
- Test-driven development support
- Automatic day generation script

## Quick Start

### 1. Use this template

Click "Use this template" on GitHub

### 2. Update the project name

Edit `Cargo.toml` and `src/main.rs`:
- In `Cargo.toml`: Change `name = "advent-of-code"` to `name = "advent-of-code-2025"`
- In `src/main.rs`: Change `use advent_of_code::days` to `use advent_of_code_2025::days`

### 3. Create a new day

```bash
./new_day.sh 1
```

This will:
- Create `src/days/day01.rs` with the template
- Update `src/days.rs` to include the module
- Update `src/main.rs` to add the day to the runner

### 4. Add your puzzle input

Save your input to `input/day01.txt`

### 5. Implement your solution

Edit `src/days/day01.rs`:
- Add example input to `EXAMPLE` constant
- Update test expectations
- Implement `part1()` and `part2()`

### 6. Run and test

```bash
# Run tests
cargo test day01 -- --nocapture

# Run actual solution
cargo run 1
```

## Utility Functions

Available in `src/lib.rs`:

```rust
// Read input file as string
let input = read_input(1);

// Parse lines into numbers
let numbers: Vec<i32> = parse_numbers(&input);

// Split by blank lines (for grouped inputs)
let groups = split_by_blank_lines(&input);
```

## Project Structure

```
.
├── Cargo.toml          # Project configuration
├── new_day.sh          # Script to generate new days
├── src/
│   ├── lib.rs          # Utility functions
│   ├── main.rs         # CLI runner
│   ├── days.rs         # Module declarations
│   └── days/
│       └── day01.rs    # Day 1 solution
└── input/
    └── day01.txt       # Puzzle input (gitignored)
```

