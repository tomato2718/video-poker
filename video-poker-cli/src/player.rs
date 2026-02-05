use crate::utils::{clear_screen, press_any_to_continue};
use dialoguer::{MultiSelect, Select};
use video_poker_core::{Card, bonus_game, video_poker};

pub struct CliPlayer {
    cards: Vec<Card>,
}

impl CliPlayer {
    pub fn new() -> Self {
        CliPlayer {
            cards: Vec::with_capacity(5),
        }
    }

    fn print_hand(&self) {
        self.cards.iter().for_each(|card| print!("{} ", card));
        println!();
    }
}

impl video_poker::Player for CliPlayer {
    fn draw(&mut self, cards: Vec<Card>) {
        self.cards.extend(cards);
        print!("Your hand is: ");
        self.print_hand();
    }

    fn exchange<F>(&mut self, exchange: F)
    where
        F: FnOnce(Vec<Card>) -> Vec<Card>,
    {
        let selection = MultiSelect::new()
            .max_length(20)
            .with_prompt("Please select the cards to keep")
            .items(self.cards.iter())
            .interact()
            .unwrap();
        let to_exchange: Vec<usize> = (0..5).filter(|i| !selection.contains(i)).collect();
        let new_cards = exchange(
            to_exchange
                .iter()
                .rev()
                .map(|i| self.cards.remove(*i))
                .collect(),
        );

        to_exchange
            .into_iter()
            .zip(new_cards.into_iter().rev())
            .for_each(|(i, card)| self.cards.insert(i, card));

        println!("Your hand after exchange is:");
        self.print_hand();
    }

    fn cards(&mut self) -> Vec<Card> {
        let mut res = Vec::with_capacity(5);
        while let Some(card) = self.cards.pop() {
            res.push(card);
        }
        res
    }
}

impl bonus_game::Player for CliPlayer {
    fn new_round(&self, current_round: usize) -> bool {
        clear_screen();
        println!(
            "Current bonus is x{}",
            2_usize.pow(current_round as u32 - 1)
        );
        Select::new()
            .with_prompt("Start a Bonus game?")
            .items(vec!["Yes", "No"])
            .default(0)
            .interact()
            .unwrap()
            == 0
    }

    fn guess(&self) -> bonus_game::Guess {
        let selection = Select::new()
            .with_prompt("The card is greater or less than 7?")
            .items(vec!["Greater", "Less"])
            .default(0)
            .interact()
            .unwrap();
        if selection == 0 {
            bonus_game::Guess::Greater
        } else {
            bonus_game::Guess::Less
        }
    }

    fn round_result(&self, card: &Card, _guess: &bonus_game::Guess, win: bool) {
        println!("The card is {card}");
        if win {
            println!("You've won the round");
        } else {
            println!("You've lost");
        }
        press_any_to_continue();
    }
}
