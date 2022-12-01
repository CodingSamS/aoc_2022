use crate::read_lines;

pub fn solve(file_path: &str) -> Result<i32, &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut result_values = vec![0, 0, 0];
        let mut current_value = 0;
        for line in lines {
            if let Ok(l) = line {
                if let Ok(value) = l.parse::<i32>() {
                    current_value += value;
                } else {
                    result_values.sort();
                    if result_values[0] < current_value {
                        result_values[0] = current_value;
                    }
                    current_value = 0;
                }
            }
        }
        Ok(result_values.iter().sum())
    } else {
        Err("no valid file")
    }
}


#[cfg(test)]
mod tests {
    use crate::puzzle_02::solve;

    #[test]
    fn test1 () {
        assert_eq!(solve("puzzle01_input_test").unwrap(), 45000);
    }
}
