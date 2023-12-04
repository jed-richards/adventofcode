use aoc::read_lines;

fn part1(file: &str) -> u32 {
    let lines = read_lines(file);
    todo!();
}

fn part2(file: &str) -> u32 {
    let lines = read_lines(file);
    todo!();
}

fn main() {
    println!("Part 1: {}", part1("data/input/dayXX.txt"));
    println!("Part 2: {}", part2("data/input/dayXX.txt"));
}

#[cfg(test)]
mod tests {
    use aoc::read_example_answer;
    use crate::*;

    #[test]
    fn test_part1() {
        let result = part1("data/example/dayXX_example.txt");
        let answer = read_example_answer("data/example/dayXX_answer.txt");
        assert_eq!(result, answer);
    }

    #[test]
    fn test_part2() {
        let result = part2("data/example/dayXX_example.txt");
        let answer = read_example_answer("data/example/dayXX_answer.txt");
        assert_eq!(result, answer);
    }
}
