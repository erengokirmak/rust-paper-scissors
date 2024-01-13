use crate::hands::Hand;
use crate::hands::RoundResult;
use crate::hands::{determine_round_result, random_hand, string_to_hand};
use std::io::Write;
use std::io::{self, stdin};

/// A model for a rock-paper-scissors game
///
/// Contains the scoreboard of the game.
/// To create a new game, use the Game::new() function.
pub struct Game {
    win_count: u32,
    draw_count: u32,
    loss_count: u32,
}

impl Game {
    /// Initialize a new game object.
    pub fn new() -> Game {
        Game {
            win_count: 0,
            draw_count: 0,
            loss_count: 0,
        }
    }

    /// Creates the game loop, regulates user input
    /// and game logic. At the end of every round,
    /// prints the current scoreboard to the output.
    pub fn play_game(&mut self) {
        loop {
            // Generate computer choice
            let comp_choice = random_hand();

            // Receive user choice
            print!("\nEnter your choice (rock, paper, or scissors) or 'q' to quit: ");
            io::stdout()
                .flush()
                .expect("Buffer should be flushed to stdout");
            let mut user_input = String::new();

            // If reading input goes wrong, there are bigger
            // problems than playing rock, paper, scissors.
            let _ = stdin().read_line(&mut user_input);

            let user_choice: Hand = match user_input.trim().to_lowercase().as_str() {
                "q" => break,
                other => match string_to_hand(other) {
                    Some(hand) => hand,
                    None => {
                        println!("Invalid input, try again.");
                        continue;
                    }
                },
            };

            // Clear the terminal
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            // Announce choices
            println!(
                "\nComputer chose {}, you chose {}.",
                &comp_choice, &user_choice
            );

            // Scoreboard logic
            match determine_round_result(user_choice, comp_choice) {
                RoundResult::Win => {
                    println!("You Win!");
                    self.win_count += 1;
                }
                RoundResult::Draw => {
                    println!("Draw!");
                    self.draw_count += 1;
                }
                RoundResult::Lose => {
                    println!("Computer Wins!");
                    self.loss_count += 1;
                }
            }

            println!(
                "\n\t   Wins | Draws | Losses
            {:3} | {:5} | {:6}",
                &self.win_count, &self.draw_count, &self.loss_count
            );
        }
    }
}
