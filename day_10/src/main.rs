use std::collections::HashMap;

fn main() {
    let register_states = day_10::read_in_file_system("input").unwrap();
    println!("Solution 1: {}", day_10::get_signal_strength_sum(vec![20, 60, 100, 140, 180, 220], &register_states));
    println!("Test Image: ");
    let register_states_test = day_10::read_in_file_system("input_test").unwrap();
    render_image(6, register_states_test);
    println!("Solution 2:");
    render_image(6, register_states);
}

fn render_image(number_of_lines: i32, register_states: HashMap<i32, i32>) {
    for line_num in 0..number_of_lines {
        for i in 1..=40 {
            let cycle_number = i + line_num * 40;
            if day_10::is_cycle_in_sprite(&cycle_number, &(i-1), &register_states).unwrap() {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}