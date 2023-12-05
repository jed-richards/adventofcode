use aoc::read_lines;

#[allow(dead_code)]
struct Solution {
    lines: Vec<String>,
    max_color_count_list: Vec<CubeCount>
}

struct CubeCount {
    count: u32,
    color: String
}

impl CubeCount {
    fn new(count: u32, color: String) -> Self {
        return CubeCount {count, color};
    }
}

impl Solution {
    fn new(lines: Vec<String>) -> Self {
        let max_color_count_list: Vec<CubeCount> = vec![
            CubeCount::new(12, "red".to_string()),
            CubeCount::new(13, "green".to_string()),
            CubeCount::new(14, "blue".to_string()),
        ];
        return Solution {lines, max_color_count_list};
    }

    fn part1(&self) -> u32 {
        //let mut sum: u32 = 0;
        //for line in &self.lines {
        //    sum = sum + self.parse_line_p1(line.to_string());
        //}

        println!("color: {} | count: {}", &self.max_color_count_list[0].color, &self.max_color_count_list[0].count);
        println!("color: {} | count: {}", &self.max_color_count_list[1].color, &self.max_color_count_list[1].count);
        println!("color: {} | count: {}", &self.max_color_count_list[2].color, &self.max_color_count_list[2].count);

        return 0;
    }

    fn part2(&self) -> u32 {
        todo!();
    }

    fn parse_line_p1(&self, line: String) -> u32 {
        todo!();
    }

    fn parse_line_p2(&self, line: String) -> u32 {
        todo!();
    }
}

fn main() {
    let lines = read_lines("data/input/day02.txt");
    //let lines = read_lines("data/example/day02/day02_part1.in");
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
        let lines = read_lines("data/example/day02/day02_part1.in");
        let solver: Solution = Solution::new(lines.unwrap());
        let result = solver.part1();
        let answer = read_example_answer("data/example/day02/day02_part1.out");
        assert_eq!(result, answer);
    }

    #[test]
    fn test_part2() {
        //let lines = read_lines("data/example/day01/day01_example2.txt");
        //let solver: Solution = Solution::new(lines.unwrap());
        //let result = solver.part2();
        //let answer = read_example_answer("data/example/day01/day01_answer2.txt");
        //assert_eq!(result, answer);
        todo!();
    }
}
