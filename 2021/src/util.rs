pub fn get_day_input(day: i32) -> String {
    std::fs::read_to_string(format!("input/day{}.txt", day)).unwrap()
}
