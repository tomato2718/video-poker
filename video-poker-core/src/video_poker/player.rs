use crate::card::Card;

pub trait Player {
    fn show_cards(&self, cards: &[Card]);

    fn exchange(&self, cards: &[Card]) -> Vec<usize>;
}
