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