use std::collections::HashMap;

pub fn question_1(lines: Vec<String>) -> i32 {
    return 0;
}

pub fn question_2(lines: Vec<String>) -> u64 {
    return 0;
}

struct Passport {
    values: HashMap<String, String>,
    mandatory_values: Vec<String>,
}

impl Passport {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};

    #[test]
    fn day_3_question_1() {
        let file = File::open("input/day_3_test.txt").unwrap();
        let reader = BufReader::new(file);
        let line_results: io::Result<Vec<String>> = reader.lines().collect();
        let soln = question_1(line_results.unwrap());
        assert_eq!(soln, 7)
    }

    #[test]
    fn day_3_question_2() {
        let file = File::open("input/day_3_test.txt").unwrap();
        let reader = BufReader::new(file);
        let line_results: io::Result<Vec<String>> = reader.lines().collect();
        let soln = question_2(line_results.unwrap());
        assert_eq!(soln, 336)
    }
}
