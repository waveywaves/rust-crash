#![allow(unused)]

use std::io; 
use rand::{Rng, rngs, thread_rng};
use rand::rngs::ThreadRng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    // println!("What is your name ?");

    // let mut name = String::new();
    // let mut age: String = String::new();
    // let greeting = "Nice to meet you !";

    // io::stdin().read_line(&mut name)
    //     .expect("Didn't Receive Input");

    // println!("What is your age ?");

    // io::stdin().read_line(&mut age)
    //     .expect("Didn't Receive Input");

    // // Shadow age
    // let mut age: u32 = age.trim().parse()
    //     .expect("Age wasn't assigned a number");

    // age = age + 1;

    // println!("Hello, {}! {}! You {} - 1 year old badass !", name.trim_end(), greeting, age);
    
    println!("{}",u32::MAX);

    println!("{}", thread_rng().gen_range(1..101));

    // Tuples
    let my_tuple: (u8, String, f64) = (26, "Vibhav".to_string(), 50_000.0);
    println!("Name : {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);

    // String
    let mut s = String::new();
    s.push('A');
    s.push_str("word");
    for word in s.split_whitespace() {
        println!("{}", word)
    }
    let s2 = s.replace("A", "Another");
    println!("{}", s2);

    let s3 = String::from("a d q s n k j i s s");
    let mut v1: Vec<char> = s3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let s4: &str = "Random string";
    let mut s5: String = s4.to_string();
    println!("{}", s5);
    let bytearr = s5.as_bytes();
    let s6 = &s5[0..6];
    println!("String length : {}", s6.len());
    let s6 = String::from("Just some");
    let s7 = String::from("words");
    let s8 = s6 + &s7;

    // Casting

    let uint8: u8 = 5;
    let uint32: u32 = (uint8 as u32) + (uint8 as u32);
    println!("{}", uint32);

    // Enums

    enum Day {
        Monday, 
        Tuesday, 
        Wednesday,
        Thursday,
        Friday, 
        Saturday, 
        Sunday
    }

    impl Day {
        fn is_weekend(&self) ->  bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today:Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates mondays"),
        _ => println!("other hatements")
    }

    println!("Is today a weekend ? {}", today.is_weekend())
}
