use crate::video_poker::Hand;

pub struct ScoreBoard {
    token: usize,
    rate: Rate,
    bonus_base: usize,
}

impl ScoreBoard {
    pub fn new(token: usize, rate: Rate, bonus_base: usize) -> Self {
        Self {
            token,
            rate,
            bonus_base,
        }
    }

    pub fn token(&self) -> usize {
        self.token
    }

    pub fn prize(&self, base: usize, hand: &Hand, bonus_rounds: Option<usize>) -> usize {
        base * self.rate.by_hand(hand)
            * bonus_rounds
                .map(|r| self.bonus_base.pow(r as u32 - 1))
                .unwrap_or(1)
    }

    pub fn cost(&mut self, cost: usize) -> Result<(), &'static str> {
        match self.token.checked_sub(cost) {
            Some(c) => {
                self.token = c;
                Ok(())
            }
            None => Err("No enough token."),
        }
    }

    pub fn apply(&mut self, base: usize, hand: Hand, bonus_rounds: usize) {
        self.token += self.prize(base, &hand, Some(bonus_rounds));
    }
}

pub struct Rate {
    pub royal_flush: usize,
    pub five_of_a_kind: usize,
    pub straight_flush: usize,
    pub four_of_a_kind: usize,
    pub full_house: usize,
    pub flush: usize,
    pub straight: usize,
    pub three_of_a_kind: usize,
    pub two_pair: usize,
    pub jacks_or_better: usize,
}

impl Rate {
    fn by_hand(&self, hand: &Hand) -> usize {
        match hand {
            Hand::RoyalFlush => self.royal_flush,
            Hand::StraightFlush => self.straight_flush,
            Hand::FiveOfAKind => self.five_of_a_kind,
            Hand::FourOfAKind => self.four_of_a_kind,
            Hand::FullHouse => self.full_house,
            Hand::Flush => self.flush,
            Hand::Straight => self.straight,
            Hand::ThreeOfAKind => self.three_of_a_kind,
            Hand::TwoPair => self.two_pair,
            Hand::JacksOrBetter => self.jacks_or_better,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DEFAULT_RATE: Rate = Rate {
        royal_flush: 500,
        five_of_a_kind: 200,
        straight_flush: 120,
        four_of_a_kind: 50,
        full_house: 10,
        flush: 7,
        straight: 5,
        three_of_a_kind: 3,
        two_pair: 2,
        jacks_or_better: 1,
    };

    #[test]
    fn cost_should_subtract_token_with_cost() {
        let mut board = ScoreBoard::new(100, DEFAULT_RATE, 2);

        board.cost(50).unwrap();
        assert_eq!(board.token(), 50);
    }

    #[test]
    fn cost_should_return_err_if_no_enough_token() {
        let mut board = ScoreBoard::new(100, DEFAULT_RATE, 2);

        assert!(board.cost(500).is_err());
    }

    #[test]
    fn apply_should_add_token_base_on_result() {
        const BASE: usize = 50;
        let test_cases = [
            ((Hand::RoyalFlush, 0), 500),
            ((Hand::FiveOfAKind, 0), 200),
            ((Hand::StraightFlush, 0), 120),
            ((Hand::FourOfAKind, 0), 50),
            ((Hand::FullHouse, 0), 10),
            ((Hand::Flush, 0), 7),
            ((Hand::Straight, 0), 5),
            ((Hand::ThreeOfAKind, 0), 3),
            ((Hand::TwoPair, 0), 2),
            ((Hand::JacksOrBetter, 0), 1),
            ((Hand::JacksOrBetter, 1), 2),
            ((Hand::JacksOrBetter, 2), 4),
            ((Hand::JacksOrBetter, 3), 8),
        ];

        for ((hand, round), expect) in test_cases {
            let mut board = ScoreBoard::new(0, DEFAULT_RATE, 2);
            board.apply(BASE, hand, round);

            assert_eq!(board.token(), BASE * expect);
        }
    }
}
