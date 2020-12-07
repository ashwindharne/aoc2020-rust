pub fn question_1(lines: Vec<String>) -> i32 {
    let rows = lines.len() as u32;
    let columns = lines[0].len() as u32;
    let traversal = ForestTraversal {
        rows: rows,
        columns: columns,
        current_pos: (0, 0),
        next_pos: (0, 0),
        slope: (1, 3),
    };
    let steps = rows / traversal.slope.0;
    let mut counter = 0;
    for step in traversal.take(steps as usize) {
        if lines[step.0 as usize].chars().nth(step.1 as usize).unwrap() == "#".parse().unwrap() {
            counter += 1;
        }
    }
    return counter;
}

pub fn question_2(lines: Vec<String>) -> u64 {
    let rows = lines.len() as u32;
    let columns = lines[0].len() as u32;
    let mut running_product: u64 = 1;
    let traversal_1 = ForestTraversal {
        rows: rows,
        columns: columns,
        current_pos: (0, 0),
        next_pos: (0, 0),
        slope: (1, 1),
    };
    let mut steps = rows / traversal_1.slope.0;
    let mut counter = 0;
    for step in traversal_1.take(steps as usize) {
        if lines[step.0 as usize].chars().nth(step.1 as usize).unwrap() == "#".parse().unwrap() {
            counter += 1;
        }
    }
    running_product *= counter;
    counter = 0;
    let traversal_2 = ForestTraversal {
        rows: rows,
        columns: columns,
        current_pos: (0, 0),
        next_pos: (0, 0),
        slope: (1, 5),
    };
    steps = rows / traversal_2.slope.0;
    counter = 0;
    for step in traversal_2.take(steps as usize) {
        if lines[step.0 as usize].chars().nth(step.1 as usize).unwrap() == "#".parse().unwrap() {
            counter += 1;
        }
    }
    running_product *= counter;
    counter = 0;
    let traversal_3 = ForestTraversal {
        rows: rows,
        columns: columns,
        current_pos: (0, 0),
        next_pos: (0, 0),
        slope: (1, 7),
    };
    steps = rows / traversal_3.slope.0;
    counter = 0;
    for step in traversal_3.take(steps as usize) {
        if lines[step.0 as usize].chars().nth(step.1 as usize).unwrap() == "#".parse().unwrap() {
            counter += 1;
        }
    }
    running_product *= counter;
    counter = 0;
    let traversal_4 = ForestTraversal {
        rows: rows,
        columns: columns,
        current_pos: (0, 0),
        next_pos: (0, 0),
        slope: (2, 1),
    };
    steps = rows / traversal_4.slope.0;
    counter = 0;
    for step in traversal_4.take(steps as usize) {
        if lines[step.0 as usize].chars().nth(step.1 as usize).unwrap() == "#".parse().unwrap() {
            counter += 1;
        }
    }
    running_product *= counter;
    counter = 0;
    let traversal_5 = ForestTraversal {
        rows: rows,
        columns: columns,
        current_pos: (0, 0),
        next_pos: (0, 0),
        slope: (1, 3),
    };
    steps = rows / traversal_5.slope.0;
    counter = 0;
    for step in traversal_5.take(steps as usize) {
        if lines[step.0 as usize].chars().nth(step.1 as usize).unwrap() == "#".parse().unwrap() {
            counter += 1;
        }
    }
    running_product *= counter;
    counter = 0;

    return running_product;
}

struct ForestTraversal {
    rows: u32,
    columns: u32,
    current_pos: (u32, u32),
    next_pos: (u32, u32),
    slope: (u32, u32),
}

impl Iterator for ForestTraversal {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        self.current_pos = self.next_pos;
        self.next_pos = (
            self.next_pos.0 + self.slope.0,
            self.next_pos.1 + self.slope.1,
        );
        if self.next_pos.1 > (self.columns - 1) {
            self.next_pos.1 -= self.columns;
        }
        Some(self.current_pos)
    }
}

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
