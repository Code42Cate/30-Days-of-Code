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

fn main() {
    let i = 4;
    let d = 4.0;
    let mut s = String::from("HackerRank ");

    let add_to_i = to_i32(read_line());
    let add_to_d = to_f64(read_line());
    let add_to_s = read_line();

    println!("{}", i + add_to_i);
    println!("{:.1}", d + add_to_d);
    s.push_str(& add_to_s);
    println!("{}", add_to_s);
}