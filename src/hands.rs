use rand::{thread_rng, Rng};
use std::{fmt, str::FromStr};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumString, FromRepr};

/// Possible hands in a rock-paper-scissors game
#[derive(PartialEq, FromRepr, EnumCount, EnumString, Debug)]
pub enum Hand {
    #[strum(serialize = "rock", serialize = "r")]
    Rock,
    #[strum(serialize = "paper", serialize = "p")]
    Paper,
    #[strum(serialize = "scissors", serialize = "s")]
    Scissors,
}

/// Possible conditions after a round
#[derive(PartialEq, Debug)]
pub enum RoundResult {
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
pub fn determine_round_result(own_hand: Hand, other_hand: Hand) -> RoundResult {
    if own_hand.beats() == other_hand {
        RoundResult::Win
    } else if other_hand.beats() == own_hand {
        RoundResult::Lose
    } else {
        RoundResult::Draw
    }
}

/// Taken in the user input and, if possible,
/// returns a valid Hand
pub fn string_to_hand(input: &str) -> Option<Hand> {
    match Hand::from_str(input) {
        Ok(hand) => Some(hand),
        Err(_) => None,
    }
}

/// Returns a random hand. If the random number generation
/// process turns out to be unsuccessful, defaults to returning a rock.
pub fn random_hand() -> Hand {
    Hand::from_repr(thread_rng().gen_range(0..Hand::COUNT)).unwrap_or(Hand::Rock)
}

#[cfg(test)]
mod tests {
    use crate::hands::RoundResult;
    use crate::hands::determine_round_result;

    use super::string_to_hand;

    use super::Hand;

    #[test]
    fn hand_selection_works() {
        use crate::hands::Hand;
        let user_input: &str = "rock";
        assert_eq!(Hand::Rock, string_to_hand(user_input).expect("Hardcoded input should return Some()."));

        let user_input: &str = "scissors";
        assert_eq!(Hand::Scissors, string_to_hand(user_input).expect("Hardcoded input should return Some().")); 

        let user_input: &str = "";
        assert_eq!(None, string_to_hand(user_input));
    }

    #[test]
    fn round_result_works() {
        assert_eq!(RoundResult::Draw, determine_round_result(Hand::Rock, Hand::Rock));

        assert_eq!(RoundResult::Win, determine_round_result(Hand::Rock, Hand::Scissors));

        assert_eq!(RoundResult::Lose, determine_round_result(Hand::Scissors, Hand::Rock));
    }
}