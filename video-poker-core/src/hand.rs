use crate::card::{Card, Rank, Suit};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum Hand {
    RoyalFlush,
    StraightFlush,
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    JacksOrBetter,
    None,
}

impl Hand {
    pub fn from_cards(cards: &[Card]) -> Self {
        let is_royals = Hand::is_royals(cards);
        let is_straight = Hand::is_straight(cards);
        let is_flush = Hand::is_flush(cards);
        let same_ranks = Hand::same_ranks(cards);
        let jokers = Hand::jokers(cards);
        let entries = Hand::count(cards);

        if is_royals && is_flush {
            Hand::RoyalFlush
        } else if is_flush && is_straight {
            Hand::StraightFlush
        } else if same_ranks + jokers == 5 {
            Hand::FiveOfAKind
        } else if same_ranks + jokers == 4 {
            Hand::FourOfAKind
        } else if Hand::is_full_house(cards) {
            Hand::FullHouse
        } else if is_flush {
            Hand::Flush
        } else if is_straight {
            Hand::Straight
        } else if same_ranks + jokers == 3 {
            Hand::ThreeOfAKind
        } else if Hand::pairs(&entries) == 2 {
            Hand::TwoPair
        } else if entries[0]
            .max(entries[10])
            .max(entries[11])
            .max(entries[12])
            + jokers
            == 2
        {
            Hand::JacksOrBetter
        } else {
            Hand::None
        }
    }

    fn is_royals(cards: &[Card]) -> bool {
        cards
            .iter()
            .filter(|card| card.suit != Suit::Joker)
            .all(|card| {
                matches!(
                    card.rank,
                    Rank::Ace | Rank::Ten | Rank::Jack | Rank::Queen | Rank::King
                )
            })
    }

    fn is_straight(cards: &[Card]) -> bool {
        let mut ranks: Vec<u8> = cards
            .iter()
            .filter(|card| card.suit != Suit::Joker)
            .map(|card| card.rank.value())
            .collect();
        ranks.sort();

        let range = ranks.last().unwrap() - ranks.first().unwrap();
        let all_diff = ranks.windows(2).all(|x| x[0] != x[1]);
        range < 5 && all_diff
    }

    fn is_flush(cards: &[Card]) -> bool {
        cards
            .iter()
            .filter(|card| card.suit != Suit::Joker)
            .collect::<Vec<&Card>>()
            .windows(2)
            .all(|slice| slice[0].suit == slice[1].suit)
    }

    fn is_full_house(cards: &[Card]) -> bool {
        cards
            .iter()
            .filter(|card| card.suit != Suit::Joker)
            .fold(Vec::new(), |mut ranks, card| {
                if !ranks.contains(&card.rank) {
                    ranks.push(card.rank.clone());
                }
                ranks
            })
            .len()
            == 2
    }

    fn same_ranks(cards: &[Card]) -> u8 {
        cards
            .iter()
            .filter(|card| card.suit != Suit::Joker)
            .fold(vec![0; 13], |mut entries, card| {
                entries[(card.rank.value() - 1) as usize] += 1;
                entries
            })
            .into_iter()
            .max()
            .unwrap()
    }

    fn jokers(cards: &[Card]) -> u8 {
        cards.iter().filter(|card| card.suit == Suit::Joker).count() as u8
    }

    fn count(cards: &[Card]) -> [u8; 13] {
        cards.iter().filter(|card| card.suit != Suit::Joker).fold(
            [0_u8; 13],
            |mut entries, card| {
                entries[(card.rank.value() - 1) as usize] += 1;
                entries
            },
        )
    }

    fn pairs(card_entries: &[u8; 13]) -> usize {
        card_entries.iter().filter(|count| count == &&2).count()
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::RoyalFlush => "Royal Flush",
                Self::StraightFlush => "Straight Flush",
                Self::FiveOfAKind => "Five of a kind",
                Self::FourOfAKind => "Four of a kind",
                Self::FullHouse => "Full House",
                Self::Flush => "Flush",
                Self::Straight => "Straight",
                Self::ThreeOfAKind => "Three Of a kind",
                Self::TwoPair => "Two pair",
                Self::JacksOrBetter => "Jacks or Better",
                Self::None => "All Other",
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::card::Rank;

    #[test]
    fn royal_flush() {
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Club
                },
            ]),
            Hand::RoyalFlush
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
            ]),
            Hand::RoyalFlush
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Joker,
                },
            ]),
            Hand::RoyalFlush
        );
    }

    #[test]
    fn straight_flush() {
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Heart
                },
            ]),
            Hand::StraightFlush
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
            ]),
            Hand::StraightFlush
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Joker
                },
            ]),
            Hand::StraightFlush
        );
    }

    #[test]
    fn five_of_a_kind() {
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
            ]),
            Hand::FiveOfAKind
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Joker
                },
            ]),
            Hand::FiveOfAKind
        );
    }

    #[test]
    fn four_of_a_kind() {
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Club
                },
            ]),
            Hand::FourOfAKind
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
            ]),
            Hand::FourOfAKind
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Joker
                },
            ]),
            Hand::FourOfAKind
        );
    }

    #[test]
    fn full_house() {
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club
                },
            ]),
            Hand::FullHouse
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
            ]),
            Hand::FullHouse
        );
    }

    #[test]
    fn flush() {
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamond
                },
            ]),
            Hand::Flush
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Joker
                },
            ]),
            Hand::Flush
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Joker
                },
            ]),
            Hand::Flush
        );
    }

    #[test]
    fn straight() {
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Club
                },
            ]),
            Hand::Straight
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
            ]),
            Hand::Straight
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Joker
                },
            ]),
            Hand::Straight
        );
    }

    #[test]
    fn three_of_a_kind() {
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Club
                },
            ]),
            Hand::ThreeOfAKind
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Joker
                },
            ]),
            Hand::ThreeOfAKind
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Joker
                },
            ]),
            Hand::ThreeOfAKind
        );
    }

    #[test]
    fn two_pair() {
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Club
                },
            ]),
            Hand::TwoPair
        );
    }

    #[test]
    fn one_pair() {
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Heart
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Club
                },
            ]),
            Hand::JacksOrBetter
        );
        assert_eq!(
            Hand::from_cards(&vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Club
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Joker
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamond
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spade
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Club
                },
            ]),
            Hand::JacksOrBetter
        );
    }
}
