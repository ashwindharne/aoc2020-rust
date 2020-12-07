use std::collections::btree_map::BTreeMap;

pub fn question_1(lines: Vec<String>) -> i32 {
    let mut counter = 0;
    for line in lines {
        let pw = Password::new(line);
        if pw.is_valid_range() {
            counter += 1;
        }
    }
    return counter;
}

pub fn question_2(lines: Vec<String>) -> i32 {
    let mut counter = 0;
    for line in lines {
        let pw = Password::new(line);
        if pw.is_valid_index() {
            counter += 1;
        }
    }
    return counter;
}

struct Password {
    min: i32,
    max: i32,
    character: char,
    text: String,
}

impl Password {
    fn new(line: String) -> Password {
        let split = line.split(' ');
        let vec: Vec<&str> = split.collect();
        let range = vec[0];
        let character = vec[1].chars().next().unwrap();
        let password_text = vec[2];
        let range_split = range.split('-');
        let range_vec: Vec<&str> = range_split.collect();
        let min = range_vec[0].parse::<i32>().unwrap();
        let max = range_vec[1].parse::<i32>().unwrap();
        return Password {
            min: min,
            max: max,
            character: character,
            text: password_text.parse().unwrap(),
        };
    }

    fn is_valid_range(&self) -> bool {
        let mut counters = BTreeMap::new();
        for c in self.text.chars() {
            *counters.entry(c).or_insert(0) += 1;
        }
        let count = match counters.get(&self.character) {
            Some(i) => *i,
            None => 0,
        };
        return self.min <= count && self.max >= count;
    }

    fn is_valid_index(&self) -> bool {
        let pw_str = self.text.to_string();
        let bytes = pw_str.as_bytes();
        return ((bytes[(self.min - 1) as usize] as char == self.character)
            || (bytes[(self.max - 1) as usize] as char == self.character))
            && !((bytes[(self.min - 1) as usize] as char == self.character)
                && (bytes[(self.max - 1) as usize] as char == self.character));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};

    #[test]
    fn day_2_question_1() {
        let file = File::open("input/day_2_test.txt").unwrap();
        let reader = BufReader::new(file);
        let line_results: io::Result<Vec<String>> = reader.lines().collect();
        let soln = question_1(line_results.unwrap());
        assert_eq!(soln, 2)
    }
    #[test]
    fn day_2_question_2() {
        let file = File::open("input/day_2_test.txt").unwrap();
        let reader = BufReader::new(file);
        let line_results: io::Result<Vec<String>> = reader.lines().collect();
        let soln = question_2(line_results.unwrap());
        assert_eq!(soln, 1)
    }
}
