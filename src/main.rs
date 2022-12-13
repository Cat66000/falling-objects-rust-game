mod game;

use std::env;

use game::game::*;
use macroquad::{prelude::*, time};

const DEFAULT_STATS_FONT_SIZE: f32 = 25.0;
const DEFAULT_STATS_X_OFFSET: f32 = -20.0;
const DEFAULT_STATS_Y_OFFSET: f32 = -100.0;
const DEFAULT_STATS_TEXT_COLOR: Color = DARKGRAY;

fn show_player_stats(players: &Vec<Player>) {
    for player in players.iter() {
        let position = player.position;

        let text_position_x = position.x + DEFAULT_STATS_X_OFFSET;
        let text_position_y = position.y + DEFAULT_STATS_Y_OFFSET;

        draw_text(
            format!("P{} - X: {} - Y: {}", player.number, position.x, position.y).as_str(),
            text_position_x,
            text_position_y,
            DEFAULT_STATS_FONT_SIZE,
            DEFAULT_STATS_TEXT_COLOR,
        );
        draw_text(
            format!("Score: {}", player.score).as_str(),
            text_position_x,
            text_position_y + 20.0,
            DEFAULT_STATS_FONT_SIZE,
            DEFAULT_STATS_TEXT_COLOR,
        );
        draw_text(
            format!("Max Score: {}", player.max_score).as_str(),
            text_position_x,
            text_position_y + 40.0,
            DEFAULT_STATS_FONT_SIZE,
            DEFAULT_STATS_TEXT_COLOR,
        );
        draw_text(
            format!("Health: {}", player.health).as_str(),
            text_position_x,
            text_position_y + 60.0,
            DEFAULT_STATS_FONT_SIZE,
            DEFAULT_STATS_TEXT_COLOR,
        );
    }
}

fn add_ball(balls: &mut Vec<Ball>, position: Vec2, ball_type: BallTypes) {
    let new_ball = Ball::new(position, ball_type);
    balls.push(new_ball);
}

fn add_player(players: &mut Vec<Player>, position: Vec2, controls: Controls) {
    let new_player = Player::new(position, controls, players.len() as i8 + 1);
    players.push(new_player);
}

async fn main_loop(players: &mut Vec<Player>, balls: &mut Vec<Ball>) {
    let mut escape_pressed_last_time = false;

    add_ball(balls, Ball::gen_ball_position(), BallTypes::Normal);
    add_player(
        players,
        Vec2 {
            x: 150.0,
            y: DEFAULT_PLAYER_Y,
        },
        Controls {
            move_left: KeyCode::Left,
            move_right: KeyCode::Right,
        },
    );
    add_player(
        players,
        Vec2 {
            x: screen_width() - 150.0 - DEFAULT_PLAYER_SIZE.width,
            y: DEFAULT_PLAYER_Y + 120.0,
        },
        Controls {
            move_left: KeyCode::Q,
            move_right: KeyCode::D,
        },
    );

    let mut last_ball_spawned = time::get_time();
    let mut pause_start_time: f64 = 0.0;
    let mut pause_time: f64 = 0.0;

    loop {
        if is_key_pressed(KeyCode::Escape) && (escape_pressed_last_time == false) {
            let new_state = if is_paused() != true {
                pause_start_time = time::get_time();
                true
            } else {
                pause_time = time::get_time() - pause_start_time;
                false
            };
            set_pause(&new_state);
            escape_pressed_last_time = true;
            println!("paused: {}", new_state);
        } else {
            escape_pressed_last_time = false;
        }
        clear_background(BLACK);
        draw_rectangle(0.0, 0.0, 800.0, 600.0, WHITE);

        // draw_rectangle(35.0, 35.0, 80.0, 80.0, RED);
        // draw_circle(75.0, 75.0, 40.0, BLACK);

        if ((time::get_time() - 50.0 - pause_time + {
            let t = get_difficulty() / 100;

            if t > 40 {
                40.0
            } else {
                t as f64
            }
        }) > last_ball_spawned)
            && (is_paused() != true)
        {
            add_difficulty(2);
            pause_time = 0.0;
            add_ball(balls, Ball::gen_ball_position(), Ball::gen_ball_type());
            last_ball_spawned = time::get_time();
        }

        for ball in balls.iter_mut() {
            ball.on_frame();
            for player in players.iter_mut() {
                if ball.check_collision_with_player(player) == true {
                    match ball.ball_type {
                        BallTypes::Normal => {
                            player.add_score(10);
                        }
                        BallTypes::Poisonous => {
                            player.remove_score(50);
                            player.remove_health(1);
                        }
                        BallTypes::Regeneration => {
                            player.add_score(10);
                            player.add_health(1);
                        }
                    }

                    ball.reset();
                }
            }

            if ball.check_collision_with_ground() == true {
                match ball.ball_type {
                    BallTypes::Normal => {
                        for player in players.iter_mut() {
                            player.remove_score(20);
                            player.remove_health(1);
                        }
                    }
                    BallTypes::Poisonous => {}
                    BallTypes::Regeneration => {
                        for player in players.iter_mut() {
                            player.remove_score(20);
                        }
                    }
                }

                ball.reset();
            }
        }
        for player in players.iter_mut() {
            player.on_frame();
        }

        show_player_stats(&players);

        if is_paused() == true {
            draw_text("Paused", 20.0, 130.0, 55.0, ORANGE);
        };
        next_frame().await
    }
}

#[macroquad::main("Falling Objects")]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut players: Vec<Player> = vec![];
    let mut balls: Vec<Ball> = vec![];

    // let player_1 = Player::new(
    //     Vec2 {
    //         x: 150.0,
    //         y: DEFAULT_PLAYER_Y,
    //     },
    //     Controls {
    //         move_left: KeyCode::Left,
    //         move_right: KeyCode::Right,
    //     },
    //     1,
    // );

    // let player_2 = Player::new(
    //     Vec2 {
    //         x: screen_width() - 150.0 - DEFAULT_PLAYER_SIZE.width,
    //         y: DEFAULT_PLAYER_Y + 120.0,
    //     },
    //     Controls {
    //         move_left: KeyCode::Q,
    //         move_right: KeyCode::D,
    //     },
    //     2,
    // );

    main_loop(&mut players, &mut balls).await;
}
