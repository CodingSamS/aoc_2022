fn main() {
    let sensor_beacon_vec = day_15::read_in_file_system("input").unwrap();
    let mut beacon_map = day_15::BeaconMap::new(day_15::manhattan_distance);
    for (sensor_coordinates, beacon_coordinates) in sensor_beacon_vec {
        beacon_map.calc_spots_at_row(sensor_coordinates, beacon_coordinates, 2000000);
    }
    println!("Solution 1: {}", beacon_map.get_spots_in_row(&2000000));
    let sensor_beacon_vec = day_15::read_in_file_system("input").unwrap();
    let mut beacon_map_performance = day_15::puzzle2::BeaconMapPerformance::new(4000000);
    for (sensor_coordinates, beacon_coordinates) in sensor_beacon_vec {
        println!("1");
        beacon_map_performance.calc_spots(sensor_coordinates, beacon_coordinates);
    }
    let (x, y) = beacon_map_performance.find_distress_signal().unwrap();
    let x_u: i128 = i128::from(x);
    let y_u: i128 = i128::from(y);
    println!("Solution 2: {}", x_u * 4000000 + y_u);
}
