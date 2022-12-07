fn main() {
    let mut filesystem = day_07::read_in_file_system("input").unwrap();
    println!("Solution 1: {}", filesystem.get_small_directory_size());
    let space_used = filesystem.get_size();
    let total_space = 70000000;
    let space_needed = 30000000;
    let space_to_free_up = space_needed - (total_space - space_used);
    let mut best_fit = 70000000;
    for size in filesystem.get_all_directory_sizes() {
        if size < best_fit && space_to_free_up <= size {
            best_fit = size;
        }
    }
    println!("Solution 2: {}", best_fit);
}
