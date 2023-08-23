use core::fmt;
use rand::{thread_rng, Rng};
use std::io::{self, stdin, Write};

enum States {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for States {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            States::Rock => write!(f, "Rock"),
            States::Paper => write!(f, "Paper"),
            States::Scissors => write!(f, "Scissors"),
        }
    }
}

fn main() -> Result<(), String> {
    let mut random = thread_rng();

    loop {
        let comp_choice: States;
        let user_choice: States;

        // Generating  computer choice
        match random.gen_range(0..3) {
            0 => comp_choice = States::Rock,
            1 => comp_choice = States::Paper,
            2 => comp_choice = States::Scissors,
            _ => {
                eprintln!("There was a serious error with the computer choice!");
                break;
            }
        }

        // Receiving user choice
        print!("Enter your choice (rock, paper, or scissors) or 'q' to quit: ");
        let mut user_input = String::new();
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => eprintln!("Something went horribly wrong! Error: {}", e),
        }
        match stdin().read_line(&mut user_input) {
            Ok(_) => match user_input.trim().to_lowercase().as_str() {
                "rock" => user_choice = States::Rock,
                "paper" => user_choice = States::Paper,
                "scissors" => user_choice = States::Scissors,
                "q" => break,
                _ => {
                    println!("You made an invalid choice, try again");
                    continue;
                }
            },
            Err(e) => {
                println!("There was an error: {}", e);
                continue;
            }
        }

        // \n is for making a space between the input message and this message
        println!(
            "\nComputer chose {}, you chose {}.",
            &comp_choice, &user_choice
        );

        // Game logic
        match (comp_choice, user_choice) {
            // Draw cases
            (States::Paper, States::Paper) => println!("Draw!"),
            (States::Rock, States::Rock) => println!("Draw!"),
            (States::Scissors, States::Scissors) => println!("Draw!"),

            // Wins for player
            (States::Paper, States::Scissors) => println!("You Win!"),
            (States::Scissors, States::Rock) => println!("You Win!"),
            (States::Rock, States::Paper) => println!("You Win!"),

            // Wins for computer
            (States::Paper, States::Rock) => println!("Computer Wins!"),
            (States::Scissors, States::Paper) => println!("Computer Wins!"),
            (States::Rock, States::Scissors) => println!("Computer Wins!"),
        }
    }
    Ok(())
}
