use std::io;    //io library to obtain user input and print result.
use rand::Rng;  //The Rng trait defines methods that random number generators
                //implement, it must be in scope for us to use those methods.
use std::cmp::Ordering; //bringing from standard library the "Ordering" type into scope
fn main() {     //the fn syntax declares a new function
    println!("Guess the number!");  //println! is a macro that print a string

    let secret_number = rand::thread_rng().gen_range(1..=100);  //"rand::thread_rng()"
                                                                //this function gives us the
                                                                //particular random number
                                                                //generator we are going to use.

    loop{   //Allowing Multiple Guesses with Looping
        println!("Please input your guess.");   //println! is a macro

        let mut guess = String::new();          //use "let mut" to make a mutable variable
                                                //Rust variable are immutable by default.
                                                //"String" is the string type from 
                                                //standard library and it is growable
                                                //UTF-8 encoded bit of text
                                                //"::new" is associated function of String

        io::stdin()     //calling stdin from io to handle user input
                    //without the line "use std::io" which call the io module
                    //this line will be "std::io::Stdin"
            .read_line(&mut guess)  //calls the "read_line" method on the standard
                                //input handle to get input from the user.
                                //the method takes "&mut guess" as argument
                                //"read_line" takes whatever the user types
                                //into standard input and append that to string
            .expect("Failed to read line"); //this lie could have been written as
                                        //"io::stdin().read_line(&mut guess).expect("Failed...");
                                        //however this will be hard to read
                                        //the job of this line is to handle
                                        //the "Result" of "read_line" which are
                                        //potential failure

        let guess: u32 = match guess.trim().parse() {  //create a new guess variabre that will
                                                        //shadow the previous but it is bound to it
                                                        //using the trim() and parse() method.
            // by using match expression, we can catch and manipulate both Result of parse()
            // method which are Ok and Err
            Ok(num) => num, //return a num
            Err(_) => continue, //catchall errors and continue to next iteration
        };
        println!("You guessed: {guess}");   //print values with println! macro

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  //Quitting After a correct guess
            }
        }
    }
}
