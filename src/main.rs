#![allow(unused)]

use std::f32::consts::PI;
use std::{io, error}; 
use rand::{Rng, rngs, thread_rng};
use rand::rngs::ThreadRng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

mod restaurant;
use crate::restaurant::order_food;


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

    // Modules

    order_food();

    // Error Handling

    let path = "lines.txt";
    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem while creating file : {:?}", error)
        }
    };

    write!(output, "Just some random words").expect(
        "Failed write to file"
    );

    let input = 
        File::open(path).unwrap();
    let buffered = 
        BufReader::new(input);

    let output2 = File::create
        ("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("rand.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file {:?}", e)
            },
            _other_error => 
                panic!("Can't create file {:?}", error),
        }
    };

    // Iterators

    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter(){
        println!("{}", val);
    }

    let mut arr_it2 = [1,2,3,4];
    let mut iterator2 = arr_it2.iter();
    println!("{:?}", iterator2.next());

    for val in iterator2{
        println!("{}", val);
    }

    // Closures

    let can_vote = |age: i32| {
        age > 18
    };

    fn use_fn<T>(a: i32, b: i32, func: T) -> i32
    where T: Fn(i32, i32) -> i32 {
        func(a,b)
    }

    let sum = |a: i32, b: i32| {
        a+b
    };

    let prod = |a: i32, b: i32| {
        a*b
    };

    println!("from sum closure : {}",use_fn(1, 2, sum));
    println!("from prod closure : {}",use_fn(1, 2, prod));

    // Allocating on the Heap

    let b_int1 = Box::new(10);
    println!("box int {b_int1}");

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    };
    
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left:None,
                right: None,
                key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> 
        Self{
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> 
        Self{
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));

    // Thread

    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread1.join().unwrap();

    pub struct Bank {
        balance: f32
    }

    // Smart Pointers

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.0 {
            println!("Current Balance : {} Withdrawal a smaller amount", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew {} Current Balance {}", amt, bank_ref.balance)
        }
    }
    
    fn customer(the_bank: &Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.0);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(
        Bank{balance: 100.0}
    ));

    let handles = (0..10).map(
        |_| {
            let bank_ref = bank.clone();
            thread::spawn( move || {
                customer(&bank_ref);
            })
        }
    );

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);

}
