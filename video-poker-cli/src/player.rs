use crate::utils::{clear_screen, press_any_to_continue};
use dialoguer::{MultiSelect, Select};
use video_poker_core::{Card, bonus_game, video_poker};

pub struct CliPlayer {}

impl CliPlayer {
    pub fn new() -> Self {
        CliPlayer {}
    }

    fn print_hand(cards: &[Card]) {
        cards.iter().for_each(|card| print!("{} ", card));
        println!();
    }
}

impl video_poker::Player for CliPlayer {
    fn show_cards(&self, cards: &[Card]) {
        clear_screen();
        print!("Your hand is: ");
        CliPlayer::print_hand(cards);
    }

    fn exchange(&self, cards: &[Card]) -> Vec<usize> {
        let selection = MultiSelect::new()
            .max_length(20)
            .with_prompt("Please select the cards to keep")
            .items(cards.iter())
            .interact()
            .unwrap();
        (0..5).filter(|i| !selection.contains(i)).collect()
    }
}

impl bonus_game::Player for CliPlayer {
    fn new_round(&self, current_round: usize) -> bool {
        clear_screen();
        println!("Current bonus is x{}", 2_usize.pow(current_round as u32));
        Select::new()
            .with_prompt("Start a Bonus game?")
            .items(vec!["Yes", "No"])
            .default(0)
            .interact()
            .unwrap()
            == 0
    }

    fn guess(&self) -> bonus_game::Guess {
        clear_screen();
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
