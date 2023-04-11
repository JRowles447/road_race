use rusty_engine::prelude::*;

struct GameState {
    high_score: u32,
    current_score: u32,
    spawn_timer: Timer,
}

// TODO set up layers for the sprites

fn main() {
    let mut game = Game::new();

    let game_state = GameState {
        high_score: 30,
        current_score: 0,
        spawn_timer: Timer::from_seconds(10.0, false),
    };

    game.add_logic(game_logic);

    // sprites
    let player = game.add_sprite("my_player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(-200.0, 0.0);
    player.collision = true;

    let barrel = game.add_sprite("barrel", SpritePreset::RacingBarrelBlue);
    barrel.translation = Vec2::new(0.0, 0.0);
    barrel.collision = true;

    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let player_reference = engine.sprites.get_mut("my_player").unwrap();
    player_reference.rotation += 0.1 * engine.delta_f32;
    // game_state.current_score += 1;
    // println!("Current score: {}", game_state.current_score);
}
