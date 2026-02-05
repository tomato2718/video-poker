use crate::card::Card;

pub trait Player {
    fn new_round(&self, current_round: usize) -> bool;

    fn guess(&self) -> Guess;

    fn round_result(&self, card: &Card, guess: &Guess, win: bool);
}

#[derive(PartialEq, Eq)]
pub enum Guess {
    Greater,
    Less,
}
