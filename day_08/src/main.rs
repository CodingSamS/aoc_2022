fn main() {
    let grid = day_08::read_in_file_system("input").unwrap();
    println!("Solution 1: {}", grid.number_of_visible_elements());
    println!("Solution 2: {}", grid.highest_scenic_score());
}
