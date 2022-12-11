use day_11::Monkey;

fn main() {
    let (mut monkey_vec, _) = day_11::read_in_file_system("input").unwrap();
    for _ in 0..20 {
        for i in 0..monkey_vec.len() {
            monkey_vec[i].inspect_items();
            let throw_vec = monkey_vec[i].throw_items();
            for (item, throw_target) in throw_vec {
                monkey_vec[throw_target].catch_item(item);
            }
        }
    }
    println!("Solution 1: {}", calc_monkey_business(monkey_vec));


    let (mut monkey_vec, ssn) = day_11::read_in_file_system("input").unwrap();
    for i in 0..monkey_vec.len() {
        let ssn_temp = ssn;
        monkey_vec[i].set_worry_calc(Box::new(move |x| x % ssn_temp));
    }
    for _ in 0..10000 {
        for i in 0..monkey_vec.len() {
            monkey_vec[i].inspect_items();
            let throw_vec = monkey_vec[i].throw_items();
            for (item, throw_target) in throw_vec {
                monkey_vec[throw_target].catch_item(item);
            }
        }
    }
    println!("Solution 2: {}", calc_monkey_business(monkey_vec))
}

fn calc_monkey_business(monkey_vec: Vec<Monkey>) -> u64 {
    let mut vals= Vec::new();
    for monkey in monkey_vec {
        vals.push(monkey.inspect_counter);
    }
    vals.sort();
    vals.pop().unwrap() * vals.pop().unwrap()
}

