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

## Day 2
Goal: Calculate the total cost for a meal with tips + taxes

Rust:
```Rust
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
```


Haskell:
```Haskell
percentageCalculator :: (Double, Double) -> Double
percentageCalculator (baseprice, percentage) = baseprice * (percentage / 100)

main = do
  mealCost <- readLn :: IO Double 
  tipPercent <- readLn :: IO Double
  taxPercent <- readLn :: IO Double
  let tip = percentageCalculator (mealCost, tipPercent)
  let tax = percentageCalculator (mealCost, taxPercent)
  print (tip + tax + mealCost)
  -- You could also do it with anonymous lambdas!:D
  -- let tip = (\x y -> x * y / 100) (mealCost) (tipPercent)
  -- let tax = (\x y -> x * y / 100) (mealCost) (taxPercent)
```
## Day 3:
Goal: 
Given an integer **n**, perform the following conditional actions:
- If **n** is odd, print Weird
- If **n** is even and in the inclusive range of 2 to 5, print Not Weird
- If **n** is even and in the inclusive range of 6 to 20, print Weird
- If **n** is even and greater than 20, print Not Weird

Haskell:
```Haskell
main = do
  n <- readLn :: IO Int
  if n `mod` 2 > 0
      then putStrLn "Weird"
      else if n > 1 && n < 6
          then putStrLn "Not Weird"
          else if n > 5 && n < 21
              then putStrLn "Weird"
              else putStrLn "Not Weird"
```
Rust:
```Rust
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
```
## Day 4:
Goal:
Too lazy to write it here, check it out yourself: https://www.hackerrank.com/challenges/30-class-vs-instance/problem
```Rust
#![allow(non_snake_case)]

struct Person {
    age: i32
}

impl Person {
    fn new(initialAge: i32) -> Person {
        let mut age = 0;
        if 0 > initialAge {
            println!("Age is not valid, setting age to 0.")
        } else {
            age = initialAge;
        }

        return Person { age: age };
        
    }
 
    fn amIOld(&self) {
        if self.age < 13 {
            println!("You are young.")
        } else if self.age < 18 {
            println!("You are a teenager.")
        } else {
            println!("You are old.")
        }
    }
 
    fn yearPasses(&mut self) {
        self.age = self.age + 1;
    }
}

fn main() {
    let T: i32 = read_line().trim().parse().unwrap();
 
    for _ in 0..T {
        let age = read_line().trim().parse().unwrap();
        let mut p = Person::new(age);
 
        p.amIOld();
 
        for _ in 0..3 {
            p.yearPasses();
        }
 
        p.amIOld();
        println!("");
    }
}

 fn read_line() -> String {
   let mut  input = String::new();
   std::io::stdin().read_line(&mut input).expect("Could not read stdin!");
    return input;
}

```
Haskell:
```Haskell
-- Haskell got disabled for this challenge because it isn't object oriented:(
```

## Day 5:
Goal: Given an Integer n, print Print 10 lines of output; each line i (where 0 < i < 11 ) contains the result of n x i in the form:
    n x i = result
    ...
    n x 10 = result

Haskell:
```Haskell
{-# LANGUAGE FlexibleInstances, UndecidableInstances, DuplicateRecordFields #-}

module Main where

import Control.Monad
import Data.Array
import Data.Bits
import Data.List
import Data.List.Split
import Data.Set
import Debug.Trace
import System.Environment
import System.IO
import System.IO.Unsafe

main :: IO()
main = do
    n <- readLn :: IO Int
    forM_ [1..10] $ \factor -> do
        putStrLn(show(n) ++ " x " ++ show(factor) ++ " = " ++ show(n*factor))
        
```
Rust:
```Rust
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
```
If you want to participate too: https://www.hackerrank.com/challenges/30-hello-world