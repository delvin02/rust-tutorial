use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng: current thread execution that is seeded by OS
    // gen_range: random number generator in range
    //
    // let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    // loop {
    //     println!("Please input your guess.");

    //     // mutable so it can be re-assigned
    //     let mut guess = String::new();

    //     // & indicates that this argument is a reference
    //     // to give ways to let multiple parts of code access one piece of data
    //     // without copying it to memory everytime
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     // rust will shadow the previous value of guess with a new one
    //     // parse returns a Result type, and Result is an enum with two variants: Ok and Err
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You guessed: {}", guess);

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    another_function(2);

    fn another_function(x: i32) {
        println!("The value of x is {x}");
    }

    // compile error
    let x = plus_one(5);

    fn plus_one(x: i32) -> i32 {
        // adding semicolon will change from an expression to a statement
        // which will not return a value
        x + 1
    }

    println!("The value of x is {x}");

    // learn loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // break applies to the inner loop
                break;
            }
            if count == 2 {
                // break applies to the outer loop depending on the label
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for loop - execute some statements for each item in a collection
    for element in a {
        println!("[element] the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
