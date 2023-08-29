use rand::{thread_rng, Rng};
use std::{fmt, str::FromStr};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumString, FromRepr};

/// Possible hands in a rock-paper-scissors game
#[derive(PartialEq, FromRepr, EnumCount, EnumString)]
pub enum Hand {
    #[strum(serialize = "rock", serialize = "r")]
    Rock,
    #[strum(serialize = "paper", serialize = "p")]
    Paper,
    #[strum(serialize = "scissors", serialize = "s")]
    Scissors,
}

/// Possible conditions after a round
pub enum HandResult {
    Win,
    Draw,
    Lose,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hand::Rock => write!(f, "Rock"),
            Hand::Paper => write!(f, "Paper"),
            Hand::Scissors => write!(f, "Scissors"),
        }
    }
}

impl Hand {
    fn beats(&self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

/// Determines the result of a round
pub fn determine_round_result(own_hand: Hand, other_hand: Hand) -> HandResult {
    if own_hand.beats() == other_hand {
        HandResult::Win
    } else if other_hand.beats() == own_hand {
        HandResult::Lose
    } else {
        HandResult::Draw
    }
}

/// Taken in the user input and, if possible,
/// returns a valid Hand
pub fn string_to_hand(input: String) -> Option<Hand> {
    match Hand::from_str(&input) {
        Ok(hand) => Some(hand),
        Err(_) => None,
    }
}

/// Returns a random hand. If the random number generation
/// process turns out to be unsuccessful, the function
/// defaults to returning a rock.
pub fn random_hand() -> Hand {
    Hand::from_repr(thread_rng().gen_range(0..Hand::COUNT)).unwrap_or(Hand::Rock)
}
