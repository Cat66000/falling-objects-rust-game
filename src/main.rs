mod game;

use game::game::*;
use macroquad::prelude::*;

fn show_score(players: &[&mut Player; 2]) {
    let score_font_size: f32 = 25.0;

    let player_1 = &players[0];
    let player_1_pos = player_1.position;
    let text = format!("P1 - X: {} - Y: {}", player_1_pos.x, player_1_pos.y);
    draw_text(text.as_str(), 20.0, 40.0, score_font_size, DARKGRAY);
    draw_text(
        format!("Score: {}", players[0].score).as_str(),
        20.0,
        60.0,
        score_font_size,
        BLACK,
    );
    draw_text(
        format!("Health: {}", players[0].health).as_str(),
        20.0,
        80.0,
        score_font_size,
        BLACK,
    );

    let player_2 = &players[1];
    let player_2_pos = player_2.position;
    let text = format!("P2 - X: {} - Y: {}", player_2_pos.x, player_2_pos.y);
    let x = 800.0 - (text.len() as f32 * score_font_size / 2.0);
    draw_text(text.as_str(), x, 40.0, score_font_size, DARKGRAY);
    draw_text(
        format!("Score: {}", players[1].score).as_str(),
        x,
        60.0,
        score_font_size,
        BLACK,
    );
    draw_text(
        format!("Health: {}", players[1].health).as_str(),
        x,
        80.0,
        score_font_size,
        BLACK,
    );
}

#[macroquad::main("Falling Objects")]
async fn main() {
    let mut player_1 = Player::new(
        Vec2 {
            x: 150.0,
            y: DEFAULT_PLAYER_Y,
        },
        Controls {
            move_left: KeyCode::Left,
            move_right: KeyCode::Right,
        },
        1,
    );

    //screen_width() - 150.0 - DEFAULT_PLAYER_WIDTH,
    // DEFAULT_PLAYER_Y + 120.0,
    let mut player_2 = Player::new(
        Vec2 {
            x: screen_width() - 150.0 - DEFAULT_PLAYER_SIZE.width,
            y: DEFAULT_PLAYER_Y + 120.0,
        },
        Controls {
            move_left: KeyCode::Q,
            move_right: KeyCode::D,
        },
        2,
    );

    let mut players: [&mut Player; 2] = [&mut player_1, &mut player_2];
    let mut balls: Vec<Ball> = vec![];

    balls.push(Ball::new(
        Vec2 { x: 80.0, y: 50.0 },
        BallTypes::Normal,
        balls.len(),
    ));

    let mut escape_pressed_last_time = false;
    loop {
        if is_key_pressed(KeyCode::Escape) && (escape_pressed_last_time == false) {
            unsafe {
                PAUSED = if PAUSED == true { false } else { true };
                escape_pressed_last_time = true;
                println!("paused: {}", PAUSED);
            }
        } else {
            escape_pressed_last_time = false;
        }
        clear_background(BLACK);
        draw_rectangle(0.0, 0.0, 800.0, 600.0, WHITE);

        // draw_rectangle(35.0, 35.0, 80.0, 80.0, RED);
        // draw_circle(75.0, 75.0, 40.0, BLACK);

        let mut balls_to_delete: Vec<usize> = vec![];

        for ball in balls.iter_mut() {
            ball.on_frame();
            for player in players.iter_mut() {
                if ball.check_collision(player) == true {
                    balls_to_delete.push(ball.index);

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
                }
            }
        }
        for player in players.iter_mut() {
            player.on_frame();
        }

        // Checks for balls to delete
        for ball_index in balls_to_delete.iter() {
            balls.remove(*ball_index);
        }

        show_score(&players);

        next_frame().await
    }
}
