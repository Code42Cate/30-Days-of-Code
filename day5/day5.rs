use std::io;

fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read new line");
    return buffer;
}

fn to_i32(source: String) -> i32 {
    source.trim().parse().expect("Failed to parse to integer")
}

fn main() {
    let n = to_i32(read_line());
    for i in 1..11 {
        println!("{} x {} = {}", n, i, n * i)
    }
}