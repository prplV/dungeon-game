#[path = "battle.rs"] mod battle;
use crate::battle::Game;
use std::io;

fn main() {
    let mut game = Game::new();
    game.start();

    //to get prompt avaliable after game finishing to see the result
    let mut a = String::new(); 
    io::stdin().read_line(&mut a).expect("error reading the line!");
}
