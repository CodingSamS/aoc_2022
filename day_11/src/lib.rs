use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;

pub struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64> ,
    throw_to: Box<dyn Fn (&u64) -> usize>,
    pub inspect_counter: u64,
    worry_calc: Box<dyn Fn(u64) -> u64>
}

impl Monkey {
    fn new(items: Vec<u64>, operation: Box<dyn Fn(u64) -> u64>, throw_to: Box<dyn Fn (&u64) -> usize>) -> Self {
        Monkey {
            items,
            operation,
            throw_to,
            inspect_counter: 0,
            worry_calc: Box::new(|x| x.saturating_div(3))
        }
    }

    pub fn inspect_items(&mut self) {
        for i in 0..self.items.len(){
            self.items[i] = (self.operation)(self.items[i]);
            self.inspect_counter += 1;
        }
    }

    pub fn throw_items(&mut self) -> Vec<(u64, usize)> {
        let mut throw_vec = Vec::new();
        loop {
            if let Some(item) = self.items.pop() {
                let item_to_throw = (self.worry_calc)(item);
                throw_vec.push((item_to_throw, (self.throw_to)(&item_to_throw)));
            } else {
                break;
            }
        }
        throw_vec
    }

    pub fn catch_item(&mut self, item: u64) {
        self.items.push(item);
    }

    pub fn set_worry_calc(&mut self, f: Box<dyn Fn(u64) -> u64>) {
        self.worry_calc = f;
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Items: {:?}\nInspect Counter: {}", self.items, self.inspect_counter)
    }
}

pub fn read_in_file_system(file_path: &str) -> Result<(Vec<Monkey>, u64), &str> {
    if let Ok(mut lines) = read_lines(file_path) {
        lazy_static! {
            static ref MONKEY_REGEX: Regex = Regex::new(r"Monkey \d+:").unwrap();
        }
        let mut monkey_vec = vec![];
        let mut ssn = 1;
        loop {
            if let Some(line) = lines.next() {
                if let Ok(l) = line {
                    if MONKEY_REGEX.is_match(&l) {
                        let items_string = lines.next().unwrap().unwrap();
                        let items = parse_items(items_string);
                        let operation_string = lines.next().unwrap().unwrap();
                        let operation = parse_operation(operation_string).unwrap();
                        let test_line_1 = lines.next().unwrap().unwrap();
                        let test_line_2 = lines.next().unwrap().unwrap();
                        let test_line_3 = lines.next().unwrap().unwrap();
                        let (throw_to, divisor) = parse_test(test_line_1, test_line_2, test_line_3);
                        ssn *= divisor;
                        monkey_vec.push(Monkey::new(items, operation, throw_to));
                    }
                }
            } else {
                break
            }
        }
        Ok((monkey_vec, ssn))
    } else {
        Err("no valid file")
    }
}

fn parse_items(line: String) -> Vec<u64> {
    lazy_static! {
        static ref ITEM_REGEX: Regex = Regex::new(r"(\d+)(, )?").unwrap();
    }
    let mut item_vec: Vec<u64> = Vec::new();
    for cap in ITEM_REGEX.captures_iter(&line) {
        let item = &cap[1];
        item_vec.push(item.parse().unwrap());
    }
    item_vec
}

fn parse_operation(operation_string: String) -> Option<Box<dyn Fn(u64) -> u64>> {
    lazy_static! {
        static ref OPERATION_REGEX: Regex = Regex::new(r"new = old (.) (old|\d+)").unwrap();
    }
    let cap = OPERATION_REGEX.captures_iter(&operation_string).next().unwrap();
    let operator = &cap[1];
    let value = &cap[2];
    if operator == "+" {
        if value == "old" {
            Some(Box::new(|x| x + x))
        } else {
            let num_value: u64 = value.parse().unwrap();
            Some(Box::new(move |x| x + num_value))
        }
    } else if operator == "*" {
        if value == "old" {
            Some(Box::new(|x| x * x))
        } else {
            let num_value: u64 = value.parse().unwrap();
            Some(Box::new(move |x| x * num_value))
        }
    } else {
        None
    }
}

fn parse_test(test_line_1: String, test_line_2: String, test_line_3: String) -> (Box<dyn Fn (&u64) -> usize>, u64) {
    lazy_static! {
        static ref DIVISIBLE_BY: Regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
        static ref TRUE_MONKEY_ID: Regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
        static ref FALSE_MONKEY_ID: Regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
    }

    let c_divisible_by = DIVISIBLE_BY.captures_iter(&test_line_1).next().unwrap();
    let divisible_by: u64 = c_divisible_by[1].parse().unwrap();
    let c_true_monkey_id = TRUE_MONKEY_ID.captures_iter(&test_line_2).next().unwrap();
    let true_monkey_id: usize = c_true_monkey_id[1].parse().unwrap();
    let c_false_monkey_id = FALSE_MONKEY_ID.captures_iter(&test_line_3).next().unwrap();
    let false_monkey_id= c_false_monkey_id[1].parse().unwrap();

    (Box::new(move |x| {
        if x % divisible_by == 0 {
            true_monkey_id
        } else {
            false_monkey_id
        }
    }), divisible_by)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::read_in_file_system;

    #[test]
    fn test_single_states() {
        let (mut monkey_vec, _) = read_in_file_system("input_test").unwrap();
        for _ in 0..20 {
            for i in 0..monkey_vec.len() {
                monkey_vec[i].inspect_items();
                let throw_vec = monkey_vec[i].throw_items();
                for (item, throw_target) in throw_vec {
                    monkey_vec[throw_target].catch_item(item);
                }
            }
        }
        assert_eq!(monkey_vec[0].inspect_counter, 101);
        assert_eq!(monkey_vec[1].inspect_counter, 95);
        assert_eq!(monkey_vec[2].inspect_counter, 7);
        assert_eq!(monkey_vec[3].inspect_counter, 105);
    }

    #[test]
    fn test_single_states_2() {
        // to do: kleinster gemeinsamer Nenner
        let (mut monkey_vec, ssn) = read_in_file_system("input_test").unwrap();
        println!("{}", ssn);
        for i in 0..monkey_vec.len() {
            let ssn_temp = ssn;
            monkey_vec[i].set_worry_calc(Box::new(move |x| x % ssn_temp));
        }

        for _ in 0..20 {
            for i in 0..monkey_vec.len() {
                monkey_vec[i].inspect_items();
                let throw_vec = monkey_vec[i].throw_items();
                for (item, throw_target) in throw_vec {
                    monkey_vec[throw_target].catch_item(item);
                }
            }
        }
        assert_eq!(monkey_vec[0].inspect_counter, 99);
        assert_eq!(monkey_vec[1].inspect_counter, 97);
        assert_eq!(monkey_vec[2].inspect_counter, 8);
        assert_eq!(monkey_vec[3].inspect_counter, 103);
    }
}