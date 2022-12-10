fn main() {
    let move_vec = day_09::read_in_file_system("input").unwrap();
    let mut walker = day_09::Walker::new(2).unwrap();
    for m in move_vec {
        walker.walk(m);
    }
    println!("Solution 1: {}", walker.tail_get_number_of_visited());

    let move_vec = day_09::read_in_file_system("input").unwrap();
    let mut walker = day_09::Walker::new(10).unwrap();
    for m in move_vec {
        walker.walk(m);
    }
    println!("Solution 2: {}", walker.tail_get_number_of_visited());
}
