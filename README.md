# Rust-Paper-Scissors

A simple game of rock, paper, scissors implemented in Rust.

## Technicalities

I had to learn match expressions and implementing the Display trait for the first time. I also had to learn how to use the rand crate. Not a hard project by any means, but it was a start on my Rust journey.

The game allows for new hands to be added without much effort. This was done with the use of 'strum' and 'strum_macros' crates. If you want to add new hands:

- Go to the [hands.rs](src/hands.rs) file.
- In the Hands enum, add your desired hand (You will
  also need to add a serialize macro to the variant. See the other variants for examples).
- You will also need to edit the beats() function to add the new hand. (See the other variants for examples).
