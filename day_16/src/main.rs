fn main() {
    let volcano_map = day_16::read_in_file_system("input").unwrap();
    let mut volcano = day_16::Volcano::new(volcano_map, 30).unwrap();
    println!("Solution 1: {}", volcano.brute_force_path_search());

    let volcano_map = day_16::read_in_file_system("input").unwrap();
    let mut volcano = day_16::Volcano::new(volcano_map, 26).unwrap();
    println!("Solution 1: {}", volcano.brute_force_path_search_with_elephant());
}
