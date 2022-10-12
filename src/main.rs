#![allow(unused)]

use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    println!("What is your name ?");

    let mut name = String::new();
    let mut age: String = String::new();
    let greeting = "Nice to meet you !";

    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");

    println!("What is your age ?");

    io::stdin().read_line(&mut age)
        .expect("Didn't Receive Input");

    // Shadow age
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");

    age = age + 1;

    println!("Hello, {}! {}! You {} - 1 year old badass !", name.trim_end(), greeting, age);
}
