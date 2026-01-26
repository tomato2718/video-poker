use crate::card::{Card, Rank, Suit};
use rand::{rng, seq::SliceRandom};

pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    pub fn default_52_cards() -> Self {
        Deck {
            deck: [Suit::Heart, Suit::Spade, Suit::Club, Suit::Diamond]
                .into_iter()
                .flat_map(|suit| {
                    [
                        Rank::Ace,
                        Rank::Two,
                        Rank::Three,
                        Rank::Four,
                        Rank::Five,
                        Rank::Six,
                        Rank::Seven,
                        Rank::Eight,
                        Rank::Nine,
                        Rank::Ten,
                        Rank::Jack,
                        Rank::Queen,
                        Rank::King,
                    ]
                    .into_iter()
                    .map(move |rank| Card {
                        rank: rank.clone(),
                        suit: suit.clone(),
                    })
                })
                .collect(),
        }
    }

    pub fn joker_54_cards() -> Self {
        let mut deck = Deck::default_52_cards();
        deck.deck.extend([
            Card {
                suit: Suit::Joker,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Joker,
                rank: Rank::Two,
            },
        ]);
        deck
    }

    pub fn push(&mut self, card: Card) {
        self.deck.push(card);
    }

    pub fn draw(&mut self) -> Result<Card, &'static str> {
        match self.deck.pop() {
            Some(card) => Ok(card),
            None => Err("Deck is empty"),
        }
    }

    pub fn shuffle(&mut self) {
        self.deck.shuffle(&mut rng());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Deck {
        pub fn inner(&self) -> Vec<Card> {
            self.deck.clone()
        }
    }

    #[test]
    fn default_52_cards_should_generate_52_cards_deck() {
        let inner = Deck::default_52_cards().inner();

        assert_eq!(inner.len(), 52);
    }

    #[test]
    fn joker_54_cards_should_contains_2_jokers() {
        let inner = Deck::joker_54_cards().inner();

        assert!(
            inner.contains(&Card {
                suit: Suit::Joker,
                rank: Rank::Ace
            }) && inner.contains(&Card {
                suit: Suit::Joker,
                rank: Rank::Two
            })
        );
    }

    #[test]
    fn push_should_put_the_card_into_deck() {
        let mut deck = Deck::default_52_cards();
        deck.push(Card {
            suit: Suit::Club,
            rank: Rank::Ace,
        });

        assert!(deck.inner().contains(&Card {
            suit: Suit::Club,
            rank: Rank::Ace,
        }))
    }

    #[test]
    fn draw_should_return_the_topmost_card() {
        let mut deck = Deck::default_52_cards();
        deck.push(Card {
            suit: Suit::Club,
            rank: Rank::Ace,
        });

        assert!(
            deck.draw().unwrap()
                == Card {
                    suit: Suit::Club,
                    rank: Rank::Ace
                }
        );
    }
}
