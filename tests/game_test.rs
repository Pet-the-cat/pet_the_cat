use pet_the_cat::game::Game;

#[test]
fn pet_one_time() {
    let mut game = Game::new();

    game.pet_cat();

    assert_eq!(game.cat_petted, 1);
}

#[test]
fn pet_one_hundred_times() {
    let mut game = Game::new();

    for _ in 0..100 {
        game.pet_cat();
    }

    assert_eq!(game.cat_petted, 100);
}

#[test]
fn pet_one_hundred_times_with_multiplier() {
    let mut game = Game::new();

    for _ in 0..100 {
        game.pet_cat();
    }

    assert_eq!(game.cat_petted, 100);
}

#[test]
fn buy_multiplier() {
    let mut game = Game::new();

    game.cat_petted = 100;

    game.buy_multiplier();

    assert_eq!(game.cat_petted, 0);
    assert_eq!(game.multiplier, 2);
}

#[test]
fn buy_petting_machine() {
    let mut game = Game::new();

    game.cat_petted = 300;

    game.buy_petting_machine();

    assert_eq!(game.cat_petted, 0);
    assert_eq!(game.petting_machine, 1);
}

#[test]
fn pet_with_petting_machine() {
    let mut game = Game::new();

    game.cat_petted = 300;

    game.buy_petting_machine();

    for _ in 0..100 {
        game.pet_cat_with_machine();
    }

    assert_eq!(game.cat_petted, 100);
}