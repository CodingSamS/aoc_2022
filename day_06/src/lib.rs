use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::VecDeque;

pub mod puzzle01 {
    use std::collections::VecDeque;

    pub fn solve(file_path: &str) -> Result<u32, &str> {
        crate::solve_routine(file_path, first_marker)
    }

    fn first_marker(s: String) -> Result<u32, &'static str> {
        let mut first_4: VecDeque<char> = VecDeque::new();
        let mut num_of_shifts = 0;
        for c in s.chars() {
            first_4.push_front(c);
            if 4 <= num_of_shifts {
                first_4.pop_back();
                if !crate::has_duplicates(&first_4) {
                    return Ok(num_of_shifts+1)
                }
            }
            num_of_shifts += 1;
        }
        Err("No start found")
    }
}

pub mod puzzle02 {
    use std::collections::VecDeque;


    pub fn solve(file_path: &str) -> Result<u32, &str> {
        crate::solve_routine(file_path, first_marker)
    }

    fn first_marker(s: String) -> Result<u32, &'static str> {
        let mut first_14: VecDeque<char> = VecDeque::new();
        let mut num_of_shifts = 0;
        for c in s.chars() {
            first_14.push_front(c);
            if 14 <= num_of_shifts {
                first_14.pop_back();
                if !crate::has_duplicates(&first_14) {
                    return Ok(num_of_shifts+1)
                }
            }
            num_of_shifts += 1;
        }
        Err("No start found")
    }

    #[cfg(test)]
    mod tests {
        use crate::puzzle02::first_marker;

        #[test]
        fn p2_test() {
            assert_eq!(first_marker(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")).unwrap(), 19);
            assert_eq!(first_marker(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")).unwrap(), 23);
            assert_eq!(first_marker(String::from("nppdvjthqldpwncqszvftbrmjlhg")).unwrap(), 23);
            assert_eq!(first_marker(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")).unwrap(), 29);
            assert_eq!(first_marker(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")).unwrap(), 26);
        }
    }

}

fn has_duplicates(vec: &VecDeque<char>) -> bool {
    let length = vec.len();
    for i in 0..length-1 {
        for j in i+1..length {
            if vec[i] == vec[j] {
                return true
            }
        }
    }
    false
}

fn solve_routine<'a>(file_path: &str, f: fn(String) -> Result<u32, &'a str>) -> Result<u32, &'a  str> {
    let file = File::open(file_path).expect("Opening File failed");
    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("Read failed");
    f(buf)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_puzzle_1_1() {
        assert_eq!(crate::puzzle01::solve("input_test_1").unwrap(), 5);
    }

    #[test]
    fn test_puzzle_1_2() {
        assert_eq!(crate::puzzle01::solve("input_test_2").unwrap(), 6);
    }

    #[test]
    fn test_puzzle_1_3() {
        assert_eq!(crate::puzzle01::solve("input_test_3").unwrap(), 10);
    }

    #[test]
    fn test_puzzle_1_4() {
        assert_eq!(crate::puzzle01::solve("input_test_4").unwrap(), 11);
    }
}
