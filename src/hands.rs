use rand::{thread_rng, Rng};
use std::fmt;
use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

pub trait Beats {
    fn beats(&self) -> Self;
}

#[derive(PartialEq, Eq, FromRepr, EnumCount)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

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

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

pub fn play_hand(own_hand: Hand, other_hand: Hand) -> HandResult {
    let (own_beats, other_beats) = (own_hand.beats(), other_hand.beats());

    if own_beats == other_hand {
        HandResult::Win
    } else if other_beats == own_hand {
        HandResult::Lose
    } else {
        HandResult::Draw
    }
}

/// Returns a random hand. If the random number generation
/// process turns out to be unsuccessful, the function
/// defaults to returning a rock.
pub fn random_hand() -> Hand {
    Hand::from_repr(thread_rng().gen_range(0..Hand::COUNT)).unwrap_or(Hand::Rock)
}
