#![allow(unused)] //to allow it not to give warnings for the unused variables

mod inner;
mod working_strs;
mod working_with_enums;
mod working_vecs;
mod hashmapy;

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io; //use std::io::*  for using it globally
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // println!("What is your name?");
    // let mut name = String::new();
    // let greeting: &str = "Nice to meet you";
    // io::stdin()
    //     .read_line(&mut name) //Result<usize,Error>
    //     .expect("Didn't Receive Input");

    // println!("Hello, {}! {}", name.trim_end(), greeting);

    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.142;

    // let mut age_input = String::new();
    // println!("What is your age?");
    // io::stdin()
    //     .read_line(&mut age_input)
    //     .expect("Didn't receive input");
    // let mut age: u32 = age_input
    //     .trim()
    //     .parse()
    //     .expect("Age wasn't assigned a number");

    // age = age + 1;

    // println!(
    //     "Hello i'm {} and my age is {} and i have {}",
    //     name, age, ONE_MIL);
    

    //                                          --------------DATA TYPES----------------

    // println!("Max u32 {}", u32::MAX);
    // println!("Max u64 {}", u64::MAX);
    // println!("Max i32 {}", i32::MAX);
    // println!("Max i64 {}", i64::MAX);
    // println!("Max f32 {}", f32::MAX);
    // println!("Max f32 {}", f32::MAX);


    // let my_grade : char = 'A';
    // let passed : bool = true ;

    //                                 ------------------RANDOM NUMBER GENERATOR--------------------------
 
    // let random_number = rand::thread_rng().gen_range(1..101);
    // println!("The random number can be delivered as {}", random_number);

    //                                 ----------------- CONDITIONALS ------------------------------------

    // let mut get_age = String::new();

    // println!("Enter your age");

    // io::stdin().read_line(&mut get_age)
    // .expect("Did't get age");
    
    // let actual_age: u32 = get_age.trim().parse()
    // .expect("Didn't get the actual value of the age");
    
    // if (actual_age <= 18){
    //     println!("Important Birthday");

    // }
    // else if ((actual_age == 25) || (actual_age == 50)) {
    //     println!("Golden Birthday");
    // }

    // else {
    //     println!("Only the Birthday");
    // }

    //                                 --------------------------BY USING MATCH-------------------------------

    // match actual_age {
    //     1..=18 => println!("Teen type Birthday"),
    //     25 | 50 => println!("Golden Birthday"),
    //     70..=u32::MAX => println!("Can't be possible"),

    //     _ => print!("Not Important"),

    // };

    // let voting_age: u32 = 18;

    // match actual_age.cmp(&voting_age){
    //     Ordering::Less => println!("Can't Vote"),
    //     Ordering::Greater=> println!("Good to go"),
    //     Ordering::Equal => println!("Has gained the right to vote"),
    // };

    // inner::inner();
    // working_strs::work();
    // working_with_enums::enums_working();
    // working_vecs::work_vectors();
    hashmapy::hasmap();



    

}
