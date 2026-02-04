use crate::player::CliPlayer;
use video_poker_core::video_poker::{Rule, VideoPoker};

use dialoguer::Select;

pub fn start() {
    clear_screen();
    let mut game = VideoPoker::new(Rule::Jokers54Cards);
    let mut player = CliPlayer::new();
    while next_round() {
        clear_screen();
        let result = game.start(&mut player);
        println!("The Result is: {}", result);
        Select::new().items([""]).default(0).interact().unwrap();
        println!();
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[2;1H", esc = 27 as char);
}

fn next_round() -> bool {
    Select::new()
        .with_prompt("Start a new game?")
        .items(vec!["Start", "Exit"])
        .default(0)
        .interact()
        .unwrap()
        == 0
}
