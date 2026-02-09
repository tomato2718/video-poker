use crate::player::CliPlayer;
use crate::utils::{clear_screen, press_any_to_continue};
use video_poker_core::{bonus_game, score_board, video_poker};

use dialoguer::Select;

pub struct CliGame {
    game: video_poker::VideoPoker,
    bonus_game: bonus_game::BonusGame,
    player: CliPlayer,
    score_board: score_board::ScoreBoard,
}

impl CliGame {
    pub fn start(&mut self) {
        while self.next_round() {
            clear_screen();

            let mut cost: usize;
            loop {
                clear_screen();
                self.show_token();
                cost = self.choose_cost();
                if self.score_board.cost(cost).is_ok() {
                    break;
                } else {
                    println!("You don't have enough tokens.");
                    press_any_to_continue();
                }
            }

            let result = self.game.start(&mut self.player);
            if let Some(hand) = result.as_ref() {
                println!("The result is: {}", hand);
                println!("The prize is: {}", self.score_board.prize(cost, hand, None));
                press_any_to_continue();
            } else {
                println!("You didn't get a hand.");
                press_any_to_continue();
                continue;
            };

            let bonus = self.bonus_game.start(&mut self.player);
            if let Some(round) = bonus.as_ref() {
                println!("The bonus is x{}", 2_usize.pow(*round as u32 - 1));
                press_any_to_continue();
            } else {
                println!("You've lost the game.");
                press_any_to_continue();
                continue;
            };

            println!(
                "The final result is {}",
                self.score_board
                    .prize(cost, result.as_ref().unwrap(), bonus)
            );
            self.score_board
                .apply(cost, result.unwrap(), bonus.unwrap());
            press_any_to_continue();
        }
    }

    fn next_round(&self) -> bool {
        clear_screen();
        if self.score_board.token() < 50 {
            println!("You don't have enough tokens to start a new round.");
            press_any_to_continue();
            println!("Game Over.");
            press_any_to_continue();
            false
        } else {
            Select::new()
                .with_prompt("Start a new game?")
                .items(vec!["Start", "Exit"])
                .default(0)
                .interact()
                .unwrap()
                == 0
        }
    }

    fn show_token(&self) {
        println!("You currently have {} tokens", self.score_board.token());
    }

    const COSTS: [usize; 5] = [50, 100, 200, 500, 1000];
    fn choose_cost(&self) -> usize {
        CliGame::COSTS[Select::new()
            .with_prompt("Choose the token to spend")
            .items(CliGame::COSTS)
            .default(0)
            .interact()
            .unwrap()]
    }
}

impl Default for CliGame {
    fn default() -> Self {
        let rate = score_board::Rate {
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
        Self {
            game: video_poker::VideoPoker::new(video_poker::Rule::Jokers54Cards),
            bonus_game: bonus_game::BonusGame::new(),
            player: CliPlayer::new(),
            score_board: score_board::ScoreBoard::new(200, rate, 2),
        }
    }
}
