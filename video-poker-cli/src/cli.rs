use crate::player::CliPlayer;
use crate::utils::{clear_screen, press_any_to_continue};
use video_poker_core::{bonus_game, video_poker};

use dialoguer::Select;

pub fn start() {
    clear_screen();
    let mut game = video_poker::VideoPoker::new(video_poker::Rule::Jokers54Cards);
    let mut bonus_game = bonus_game::BonusGame::new();
    let mut player = CliPlayer::new();
    while next_round() {
        clear_screen();
        let result = game.start(&mut player);
        println!("The result is: {}", result);
        press_any_to_continue();
        println!();
        if result != video_poker::Hand::None {
            match bonus_game.start(&mut player) {
                Some(round) => println!("The final bonus is x{}", 2_usize.pow(round as u32 - 1)),
                None => println!("You've lost the game."),
            };
            press_any_to_continue();
        }
    }
}

fn next_round() -> bool {
    clear_screen();
    Select::new()
        .with_prompt("Start a new game?")
        .items(vec!["Start", "Exit"])
        .default(0)
        .interact()
        .unwrap()
        == 0
}
