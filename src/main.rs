extern crate rand;

use std::io;
use std::cmp;

fn main() {
    println!("Guess the number!");
    let secret_number = (rand::random::<u32>() % 100)+1;
    println!("Please input your guess.");
    let mut guess = String::new();
    let mut i = 5;
    while i > 0 {
    	guess.clear();
        io::stdin().read_line(&mut guess).ok().expect("failed to read line");
        let guess: u32 = guess.trim().parse().ok().expect("Please type a number!");
        match guess.cmp(&secret_number) {
            cmp::Ordering::Less    => println!("Too small!"),
            cmp::Ordering::Greater => println!("Too big!"),
            cmp::Ordering::Equal   => {println!("You WIN! It is {}", secret_number);break;},
        }
        i-=1;
        println!("{} remain",i);
    }
    if i <=0 {
    	println!("You FAIL! It is {}", secret_number);
    }
}