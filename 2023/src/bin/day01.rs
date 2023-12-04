use aoc::read_lines;

fn parse_line(line: String) -> u32 {
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

fn part1(file: &str) -> u32 {
    let lines = read_lines(file);

    let mut sum: u32 = 0;
    for line in lines.unwrap() {
        sum = sum + parse_line(line);
    }

    return sum;
}

fn part2(file: &str) -> u32 {
    let lines = read_lines(file);

    let mut sum: u32 = 0;
    for line in lines.unwrap() {
        sum = sum + parse_line(line);
    }

    return sum;
}

fn main() {
    println!("Part 1: {}", part1("data/input/day01.txt"));
    println!("Part 2: {}", part2("data/input/day01.txt"));
}

#[cfg(test)]
mod tests {
    use aoc::read_example_answer;
    use crate::*;

    #[test]
    fn test_part1() {
        let result = part1("data/example/day01_example.txt");
        let answer = read_example_answer("data/example/day01_answer.txt");
        assert_eq!(result, answer);
    }

    #[test]
    fn test_part2() {
        let result = part1("data/example/day01_example.txt");
        //let answer = read_example_answer("data/example/day01_answer.txt");
        let answer = 0;
        assert_eq!(result, answer);
    }
}
