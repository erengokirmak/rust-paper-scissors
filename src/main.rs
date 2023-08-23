use core::fmt;
use rand::{thread_rng, Rng};
use std::io::{self, stdin, Write};

/// The choices the computer or the user can make
/// while playing rock paper scissors
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

    // wins, draws, losses
    let mut scores: (u8, u8, u8) = (0, 0, 0);

    loop {
        let comp_choice: States;
        let user_choice: States;

        // Generate computer choice
        match random.gen_range(0u8..3u8) {
            0u8 => comp_choice = States::Rock,
            1u8 => comp_choice = States::Paper,
            2u8 => comp_choice = States::Scissors,
            _ => {
                eprintln!("There was a serious error with the computer choice!");
                break;
            }
        }

        // Receive user choice
        print!("\nEnter your choice (rock, paper, or scissors) or 'q' to quit: ");
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
                    println!("You made an invalid choice, try again\n");
                    continue;
                }
            },
            Err(e) => {
                println!("There was an error: {}", e);
                continue;
            }
        }

        // Announce choices
        println!(
            "\nComputer chose {}, you chose {}.",
            &comp_choice, &user_choice
        );

        // Game logic
        match (comp_choice, user_choice) {
            // Draw cases
            (States::Paper, States::Paper)
            | (States::Rock, States::Rock)
            | (States::Scissors, States::Scissors) => {
                println!("Draw!");
                scores.1 += 1;
            }

            // Wins for player
            (States::Paper, States::Scissors)
            | (States::Rock, States::Paper)
            | (States::Scissors, States::Rock) => {
                println!("You Win!");
                scores.0 += 1;
            }

            // Wins for computer
            (States::Paper, States::Rock)
            | (States::Scissors, States::Paper)
            | (States::Rock, States::Scissors) => {
                println!("Computer Wins!");
                scores.2 += 1;
            }
        }

        // Print scores
        println!(
            "Wins: {}, Draws: {}, Losses: {}.",
            scores.0, scores.1, scores.2
        );
    }
    Ok(())
}
