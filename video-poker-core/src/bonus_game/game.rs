use super::player::{Guess, Player};
use crate::card::Card;
use crate::deck::Deck;

pub struct BonusGame {
    deck: Deck,
}

impl BonusGame {
    pub fn new() -> Self {
        BonusGame {
            deck: Deck::default_52_cards(),
        }
    }

    pub fn start(&mut self, player: &mut impl Player) -> Option<usize> {
        let mut current_round = 1;
        while player.new_round(current_round) {
            self.deck.shuffle();
            let card = self.deck.draw().unwrap();
            let guess = player.guess();
            let is_correct = BonusGame::guess_is_correct(&card, &guess);
            player.round_result(&card, &guess, is_correct);
            self.deck.push(card);
            if is_correct {
                current_round += 1;
            } else {
                return None;
            }
        }
        Some(current_round)
    }

    fn guess_is_correct(card: &Card, guess: &Guess) -> bool {
        let v = card.rank.value();
        if v > 7 {
            guess == &Guess::Greater
        } else if v < 7 {
            guess == &Guess::Less
        } else {
            true
        }
    }
}

impl Default for BonusGame {
    fn default() -> Self {
        Self::new()
    }
}
