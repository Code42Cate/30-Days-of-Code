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

    if n % 2 != 0 { 
        println!("Weird");
    } else if n > 1 && 6 > n {
        println!("Not Weird");
    } else if n > 5 && n < 21 {
        println!("Weird");
    } else {
        println!("Not Weird");
    }
}