mod hands;

use hands::{play_hand, random_hand, Hand, HandResult};

use std::io::{self, stdin, Write};

use crate::hands::user_input_to_hand;

fn main() {
    // wins, draws, losses
    let mut scores: (u8, u8, u8) = (0, 0, 0);

    loop {
        // Generate computer choice
        let comp_choice = random_hand();

        // Receive user choice
        print!("\nEnter your choice (rock, paper, or scissors) or 'q' to quit: ");
        let mut user_input = String::new();
        io::stdout()
            .flush()
            .expect("Buffer should be flushed to stdout");
        let user_choice: Hand = match stdin().read_line(&mut user_input) {
            Ok(_) => {
                if user_input.trim().to_lowercase().as_str() == "q" {
                    break;
                }
                match user_input_to_hand(user_input.trim().to_lowercase()) {
                    Some(hand) => hand,
                    None => {
                        println!("Invalid input, try again.");
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("There was an error: {}", e);
                continue;
            }
        };

        // Announce choices
        println!(
            "\nComputer chose {}, you chose {}.",
            &comp_choice, &user_choice
        );

        // Game logic
        match play_hand(user_choice, comp_choice) {
            HandResult::Win => {
                println!("You Win!");
                scores.0 += 1;
            }
            HandResult::Draw => {
                println!("Draw!");
                scores.1 += 1;
            }
            HandResult::Lose => {
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
}
