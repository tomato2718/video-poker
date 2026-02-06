use super::hand::Hand;
use super::player::Player;
use crate::card::Card;
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
        let mut players_deck: Vec<Card> = (0..5).map(|_| self.deck.draw().unwrap()).collect();
        player.show_cards(&players_deck);

        let indice_to_exchange = player.exchange(&players_deck);
        indice_to_exchange
            .iter()
            .rev()
            .for_each(|i| self.deck.push(players_deck.remove(*i)));
        self.deck.shuffle();
        indice_to_exchange
            .into_iter()
            .for_each(|i| players_deck.insert(i, self.deck.draw().unwrap()));
        player.show_cards(&players_deck);

        let hand = Hand::from_cards(&players_deck);
        players_deck
            .into_iter()
            .for_each(|card| self.deck.push(card));
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
