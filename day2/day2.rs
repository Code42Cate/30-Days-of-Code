use std::io;

fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read new line");
    return buffer;
}

fn to_i32(source: String) -> i32 {
    source.trim().parse().expect("Failed to parse to integer")
}

fn to_f64(source: String) -> f64 {
    source.trim().parse().expect("Failed to parse to float")
}

fn solve(meal_cost: f64, tip_percent: i32, tax_percent: i32) {

    let result = meal_cost * ((tip_percent as f64 / 100.0 ) + ( tax_percent as f64 / 100.0 ));

    println!("{}", result);
}

fn main() {
    let meal_cost = to_f64(read_line());
    let tip_percent = to_i32(read_line());
    let tax_percent = to_i32(read_line());

    solve(meal_cost, tip_percent, tax_percent);
}