# 30 Days of Code 2019 by HackerRank
This is the first time that I am participating in this challenge and since I didn't learn a new language in some time I thought it might be appropriate to start with this challenge!

I already know Java, JavaScript, Python, C, Go and PHP, so I thought a language which is fundamentally different, like Haskell would be fun. I'm also trying Rust, mainly because everybody tells me it's a great language to work with:D

So without further blablabla, I am going to document my solutions to this journey here:

## Day 0:

Goal: Print "Hello, World." on the first line and the input from stdin on the second line!

Rust:
```Rust
use std::io;
fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{}", "Hello, World.");
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
}
```
Haskell:

```Haskell
main = do
    z <- getLine
    putStrLn "Hello, World."
    putStr z
```
## Day 1:

Goal: Initialize 3 values, Integer, Double and String. Then read an Integer, Double and String and add those together and print them in the end.

Rust:
```Rust
use std::io;

fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read new line");
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
    s.push_str(&add_to_s);
    println!("{}", add_to_s);
}
```
Haskell:
```Haskell
main = do
  let a = 3
  let b = 4.0
  let c = "HackerRank "  
  int <- getLine
  print $ a + (read int :: Int)
  double <- getLine
  print $ b + (read double :: Double)
  string <-getLine
  putStrLn(c ++ string)
```



If you want to participate too: https://www.hackerrank.com/challenges/30-hello-world