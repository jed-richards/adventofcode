use std::collections::HashMap;

use aoc::read_lines;

struct Solution {
    lines: Vec<String>,
    num_map: HashMap<String, u32>
}

impl Solution {
    fn new(lines: Vec<String>) -> Self {
        let num_map: HashMap<String, u32> = HashMap::from([
            ("one".to_string(), 1),
            ("two".to_string(), 2),
            ("three".to_string(), 3),
            ("four".to_string(), 4),
            ("five".to_string(), 5),
            ("six".to_string(), 6),
            ("seven".to_string(), 7),
            ("eight".to_string(), 8),
            ("nine".to_string(), 9),
        ]);
        return Solution {lines, num_map};
    }

    fn part1(&self) -> u32 {
        let mut sum: u32 = 0;
        for line in &self.lines {
            sum = sum + self.parse_line_p1(line.to_string());
        }

        return sum;
    }

    fn part2(&self) -> u32 {
        let mut sum: u32 = 0;
        for line in &self.lines {
            sum = sum + self.parse_line_p2(line.to_string());
        }

        return sum;
    }

    fn parse_line_p1(&self, line: String) -> u32 {
        let mut first: Option<u32> = None;
        let mut second: Option<u32> = None;
        for ch in line.chars() {
            if ch.is_digit(10) {
                if first.is_none() {
                    first = ch.to_digit(10);
                }
                else {
                    second = ch.to_digit(10);
                }
            }
        }

        let val = match (first, second) {
            (Some(x), Some(y)) => (x*10) + y,
            (Some(x), None) => (x*10) + x,
            _ => panic!("Error")
        };
        return val;
    }

    fn parse_line_p2(&self, line: String) -> u32 {
        let mut first: Option<u32> = None;
        let mut second: Option<u32> = None;

        let line_length = line.len();
        for (i, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                if first.is_none() {
                    first = ch.to_digit(10);
                }
                else {
                    second = ch.to_digit(10);
                }
            }
            else {
                for (num_str, val) in &self.num_map {
                    let len_from_end = line_length - i;
                    if num_str.len() > len_from_end {
                        continue;
                    }
                    let new_str = &line[i..i+num_str.len()];
                    if num_str == new_str {
                        if first.is_none() {
                            first = Some(*val);
                        }
                        else {
                            second = Some(*val);
                        }
                    }
                }
            }
        }

        let val = match (first, second) {
            (Some(x), Some(y)) => (x*10) + y,
            (Some(x), None) => (x*10) + x,
            _ => panic!("Error")
        };

        return val;
    }
}

fn main() {
    let lines = read_lines("data/input/day01.txt");
    let solver: Solution = Solution::new(lines.unwrap());

    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}

#[cfg(test)]
mod tests {
    use aoc::read_example_answer;
    use crate::*;

    #[test]
    fn test_part1() {
        let lines = read_lines("data/example/day01/day01_example.txt");
        let solver: Solution = Solution::new(lines.unwrap());
        let result = solver.part1();
        let answer = read_example_answer("data/example/day01/day01_answer.txt");
        assert_eq!(result, answer);
    }

    #[test]
    fn test_part2() {
        let lines = read_lines("data/example/day01/day01_example2.txt");
        let solver: Solution = Solution::new(lines.unwrap());
        let result = solver.part2();
        let answer = read_example_answer("data/example/day01/day01_answer2.txt");
        assert_eq!(result, answer);
    }
}
