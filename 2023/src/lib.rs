mod common {
    pub use std::fs::File;
    pub use std::io::{BufReader, Result, BufRead};

    pub fn read_lines(path: &str) -> Result<Vec<String>> {
        // Create File object
        let file = File::open(path)?;

        // Create a buffered reader to efficiently read lines
        let reader = BufReader::new(file);

        // Read all lines into a vector of strings
        let lines: Result<Vec<String>> = reader.lines().collect();
        return lines;
    }

    pub fn read_example_answer(path: &str) -> u32 {
        let answer_lines = match read_lines(path) {
            Ok(lines) => lines,
            Err(_) => panic!("Failed to read example answer file."),
        };
        let answer = answer_lines
            .get(0)
            .unwrap()
            .parse::<u32>();

        return answer.unwrap();
    }
}

pub use common::*;
