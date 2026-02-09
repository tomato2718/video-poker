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

    pub fn start(&mut self, player: &mut impl Player) -> Option<Hand> {
        self.deck.shuffle();
        let mut players_deck = self.create_deck();
        player.show_cards(&players_deck);

        let indice_to_exchange = player.exchange(&players_deck);
        self.exchange_cards(&mut players_deck, indice_to_exchange);
        player.show_cards(&players_deck);

        let hand = Hand::from_cards(&players_deck);
        self.push_back_to_deck(players_deck);
        hand
    }

    fn create_deck(&mut self) -> Vec<Card> {
        (0..5).map(|_| self.deck.draw().unwrap()).collect()
    }

    fn exchange_cards(&mut self, deck: &mut Vec<Card>, indice_to_exchange: Vec<usize>) {
        indice_to_exchange
            .iter()
            .rev()
            .for_each(|i| self.deck.push(deck.remove(*i)));
        self.deck.shuffle();
        indice_to_exchange
            .into_iter()
            .for_each(|i| deck.insert(i, self.deck.draw().unwrap()));
    }

    fn push_back_to_deck(&mut self, players_deck: Vec<Card>) {
        players_deck
            .into_iter()
            .for_each(|card| self.deck.push(card));
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
