use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

pub mod puzzle01 {
    pub fn solve(file_path: &str) -> Result<u32, &str> {
        crate::solve_routine(file_path, full_containment)
    }

    fn full_containment(vec1: &Vec<u32>, vec2: &Vec<u32>) -> bool {
        if vec1.get(0).unwrap() <= vec2.get(0).unwrap() && vec2.get(1).unwrap() <= vec1.get(1).unwrap(){
            true
        } else if vec2.get(0).unwrap() <= vec1.get(0).unwrap() && vec1.get(1).unwrap() <= vec2.get(1).unwrap() {
            true
        } else {
            false
        }
    }
}

pub mod puzzle02 {
    pub fn solve(file_path: &str) -> Result<u32, &str> {
        crate::solve_routine(file_path, overlap)
    }

    fn overlap(vec1: &Vec<u32>, vec2: &Vec<u32>) -> bool {
        let range_1 = vec1[0]..=vec1[1];
        let range_2 = vec2[0]..=vec2[1];
        if range_1.contains(&vec2[0]) || range_1.contains(&vec2[1]) ||
            range_2.contains(&vec1[0]) || range_2.contains(&vec1[1]) {
            true
        } else {
            false
        }
    }
}

fn solve_routine(file_path: &str, f: fn(vec1: &Vec<u32>, vec2: &Vec<u32>) -> bool) -> Result<u32, &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut sum = 0;
        for line in lines {
            if let Ok(l) = line{
                let ranges_vec: Vec<&str> = l.split(',').collect();
                let mut sections_vec: Vec<Vec<u32>> = Vec::new();
                for range in ranges_vec {
                    let sections: Vec<u32> = range.split('-').map(|s| u32::from_str(s).unwrap()).collect();
                    if sections.len() == 2 {
                        sections_vec.push(sections);
                    }
                }
                for i in 0..sections_vec.len() {
                    for j in (i+1)..sections_vec.len() {
                        if f(sections_vec.get(i).unwrap(), sections_vec.get(j).unwrap()) {
                            sum += 1;
                        }
                    }
                }
            }
        }
        Ok(sum)
    } else {
        Err("no valid file")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_puzzle_1() {
        assert_eq!(crate::puzzle01::solve("input_test").unwrap(), 2)
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(crate::puzzle02::solve("input_test").unwrap(), 4)
    }
}
