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
        Err(error) => println!("error: {}", error);
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


If you want to participate too: https://www.hackerrank.com/challenges/30-hello-world