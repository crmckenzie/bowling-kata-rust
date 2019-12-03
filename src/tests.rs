use crate::*;

#[test]
fn all_gutters() {
    let mut game = Game::default();
    for _ in 0..20 {
        game.roll(0);
    }

    assert_eq!(0, game.score());
}

#[test]
fn all_ones() {
    let mut game = Game::default();
    for _ in 0..20 {
        game.roll(1);
    }

    assert_eq!(20, game.score());   
}

fn roll_spare(game: &mut Game) {
    game.roll(5);
    game.roll(5);
}

fn roll_strike(game: &mut Game) {
    game.roll(10);
}

#[test]
fn spare() {
    let mut game = Game::default();
    roll_spare(&mut game);
    game.roll(3);

    assert_eq!(16, game.score());
}

#[test]
fn strike() {
    let mut game = Game::default();
    roll_strike(&mut game);
    game.roll(3);
    game.roll(4);

    assert_eq!(10, game.rolls[0]);
    assert_eq!(3, game.rolls[1]);
    assert_eq!(4, game.rolls[2]);
    assert_eq!(0, game.rolls[3]);

    assert_eq!(24, game.score());
}

#[test]
fn perfect_game() {
    let mut game = Game::default();
    for _ in 0..12 {
        game.roll(10);
    }
    assert_eq!(300, game.score());
}

