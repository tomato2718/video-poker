use super::hand::Hand;
use super::player::Player;
use crate::deck::Deck;

pub struct VideoPoker {
    deck: Deck,
}

impl VideoPoker {
    pub fn new(rule: Rule) -> Self {
        VideoPoker {
            deck: rule.create_deck(),
        }
    }

    pub fn start(&mut self, player: &mut impl Player) -> Hand {
        self.deck.shuffle();
        player.draw(
            (0..5)
                .map(|_| self.deck.draw().expect("Should contains enough cards"))
                .collect(),
        );
        player.exchange(|cards| {
            let count = cards.len();
            cards.into_iter().for_each(|card| self.deck.push(card));
            self.deck.shuffle();
            (0..count)
                .map(|_| self.deck.draw().expect("Should contains enough cards"))
                .collect()
        });
        let cards = player.cards();
        let hand = Hand::from_cards(&cards);
        cards.into_iter().for_each(|card| self.deck.push(card));
        hand
    }
}

pub enum Rule {
    Default52Cards,
    Jokers54Cards,
}

impl Rule {
    fn create_deck(&self) -> Deck {
        match self {
            Self::Default52Cards => Deck::default_52_cards(),
            Self::Jokers54Cards => Deck::joker_54_cards(),
        }
    }
}
