#![allow(unused)]

use std::f32::consts::PI;
use std::io; 
use rand::{Rng, rngs, thread_rng};
use rand::rngs::ThreadRng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;


fn say_hello() {
    println!("Hello");
}

fn get_sum(x: i32, y: i32) -> i32 {
    println!("{} {} {}",x,y,x+y);
    return x+y;
}

fn get_2(x: i32) -> (i32, i32) {
    return (x+1, x+2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &i in list.iter() {
        sum += &i;
    }
    return sum;
}

fn sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    return x;
}

fn change_str(name: &mut String){
    name.push_str(" is happy");
    println!("Message : {}", name);
}

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

    println!("Is today a weekend ? {}", today.is_weekend());

    // Vectors

    let vec1: Vec<u32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);
    println!("{}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("{}", second),
        None => println!("No second value"),
    }
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("{}", vec2.len());
    println!("{:?}", vec2.pop());

    // Functions 

    say_hello();
    println!("{}",get_sum(5,4));

    let (val1, val2) = get_2(3);
    println!("{} {} ", val1, val2);
    println!("{}", sum_list(&vec2));

    // Generics

    println!("{}", sum_gen(50_000.2, 120.10));
    println!("{}", sum_gen(5, 12));

    // Ownership

    let str1: String = String::from("World");
    let mut mutstr1: String = String::from("World");
    let str2 : String = str1.clone();
    println!("Hello {}", str1); 

    let str3 = print_return_str(str1);
    println!("str3 = {}", str3);
    change_str(&mut mutstr1);

    //HashMap

    let mut heroes: HashMap<&str, &str> = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruise Wayme");
    heroes.insert("The Flash", "Barry Allen");

    for(k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    println!("Length : {}", heroes.len());
    if heroes.contains_key("Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some (x) => println!("Batman is a hero !"),
            None => println!("Batman is not a hero")
        }
    }

    // Struct

    struct Customer{
        name: String,
        address: String,
        balance: f32
    }

    let mut bob: Customer = Customer{
        name: String::from("bob"),
        address: String::from("house"),
        balance: 234.50
    };

    bob.address = String::from("123 st.");

    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle{
        length: 4,
        height: 12.4,
    };

    // Traits

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rect {
        length: f32,
        width: f32,
    };

    struct Circle {
        length: f32,
        width: f32
    };

    impl Shape for Rect {
        fn new(length: f32, width: f32) -> Rect {
            return Rect{length, width};
        }

        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }

        fn area(&self) -> f32 {
            return (self.length/ 2.0).powf(2.0) * PI;
        }
    }

    let rect: Rect = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area : {}", rect.area());
    println!("Circ Area : {}", circ.area());

}
