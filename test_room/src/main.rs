/*
 * @Author       : Shunyi
 * @Date         : 2021-09-16 09:35:17
 * @LastEditors  : Shunyi
 * @LastEditTime : 2021-09-16 14:25:37
 * @Blog         : https://shunyi.fun/
 * @FilePath     : \Rust\test_room\src\main.rs
 */
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret_number is:{}", secret_number);
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = guess.trim().parse().expect("Please type a number.");
        println!("Your guess number:{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You WIn!");
                break;
            }
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too Small"),
        }
    }
}
