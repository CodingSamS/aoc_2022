fn main() {
    let point_vec = day_18::read_in_file_system("input").unwrap();
    let cube_grid = day_18::CubeGrid::new(point_vec);
    println!("Solution 1: {}", cube_grid.get_surface_area());
}
