fn main() {
    let map = day_14::read_in_file_system("input").unwrap();
    let mut cave = day_14::cave1::Cave::new(map);
    let mut sand_counter = 0;
    while cave.pour_in_sand() {
        sand_counter += 1;
    }
    println!("Solution 1: {}", sand_counter);

    let map = day_14::read_in_file_system("input").unwrap();
    let mut cave = day_14::cave2::Cave::new(map);
    let mut sand_counter = 0;
    while cave.pour_in_sand() {
        sand_counter += 1;
    }
    println!("Solution 2: {}", sand_counter);
}
