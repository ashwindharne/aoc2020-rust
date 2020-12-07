use std::collections::HashSet;

pub fn question_1(lines: Vec<String>) -> i32 {
    return product_of_addends(2020, lines.as_slice());
}

pub fn question_2(lines: Vec<String>) -> i32 {
    for (i, line) in lines.iter().enumerate() {
        let num = line.parse::<i32>().unwrap();
        let product = product_of_addends(2020 - num, &lines[i..]);
        if product != 0 {
            return num * product;
        }
    }
    return 0;
}

fn product_of_addends(sum: i32, string_vec_slice: &[String]) -> i32 {
    let mut solution_set = HashSet::<i32>::new();
    for num_str in string_vec_slice {
        let num = num_str.parse::<i32>().unwrap();
        if solution_set.contains(&num) {
            return (sum - num) * num;
        }
        solution_set.insert(sum - num);
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};

    #[test]
    fn day_1_question_1() {
        let file = File::open("input/day_1_test.txt").unwrap();
        let reader = BufReader::new(file);
        let line_results: io::Result<Vec<String>> = reader.lines().collect();
        let soln = question_1(line_results.unwrap());
        assert_eq!(soln, 514579)
    }

    #[test]
    fn day_1_question_2() {
        let file = File::open("input/day_1_test.txt").unwrap();
        let reader = BufReader::new(file);
        let line_results: io::Result<Vec<String>> = reader.lines().collect();
        let soln = question_2(line_results.unwrap());
        assert_eq!(soln, 241861950)
    }
}
