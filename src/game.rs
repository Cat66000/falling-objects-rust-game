pub mod game {
    use macroquad::{prelude::*, rand::gen_range};

    pub static mut DIFFICULTY: i32 = 1;
    pub static mut PAUSED: bool = false;

    pub fn get_difficulty() -> i32 {
        unsafe { DIFFICULTY }
    }
    pub fn set_difficulty(new_difficulty: i32) {
        unsafe {
            DIFFICULTY = new_difficulty;
        }
    }
    pub fn add_difficulty(difficulty_to_add: i32) {
        unsafe {
            DIFFICULTY += difficulty_to_add;
        }
    }

    pub fn is_paused() -> bool {
        unsafe { PAUSED }
    }
    pub fn set_pause(new_state: &bool) {
        unsafe {
            PAUSED = *new_state;
        }
    }

    pub const MAX_HEALTH: i8 = 3;

    pub const DEFAULT_PLAYER_SIZE: Size = Size {
        width: 90.0,
        height: 70.0,
    };
    pub const DEFAULT_PLAYER_Y: f32 = 360.0;
    pub const DEFAULT_PLAYER_HEALTH: i8 = 3;
    pub const DEFAULT_PLAYER_SPEED: f32 = 7.0;

    pub const DEFAULT_BALL_RADIUS: f32 = 30.0;
    pub const DEFAULT_BALL_Y: f32 = 40.0;

    pub const LEFT_MAP_BORDER: f32 = 40.0;
    pub const RIGHT_MAP_BORDER: f32 = 800.0 - LEFT_MAP_BORDER;
    pub const GROUND_MAP_BORDER: f32 = 600.0;
    // #[derive(Clone, Copy)]
    pub struct Size {
        pub width: f32,
        pub height: f32,
    }

    pub struct Controls {
        pub move_left: KeyCode,
        pub move_right: KeyCode,
    }

    pub struct Player {
        pub position: Vec2,
        pub number: i8,
        pub size: Size,
        pub health: i8,
        pub score: i32,
        pub max_score: i32,
        //
        pub controls: Controls,
    }

    impl Player {
        pub fn new(position: Vec2, controls: Controls, number: i8) -> Self {
            let health: i8 = DEFAULT_PLAYER_HEALTH;
            let size = DEFAULT_PLAYER_SIZE;

            Self {
                position,
                number,
                size,
                health,
                score: 0,
                max_score: 0,
                controls,
            }
        }

        pub fn set_controls(&mut self, controls: Controls) -> &Self {
            self.controls = controls;
            self
        }

        pub fn set_position(&mut self, new_position: Vec2) -> &Self {
            self.position = new_position;

            self
        }

        pub fn set_health(&mut self, health: i8) -> &Self {
            self.health = health;
            self
        }

        pub fn add_health(&mut self, health_to_add: i8) -> &Self {
            self.health += health_to_add;
            if self.health > MAX_HEALTH {
                self.health = MAX_HEALTH
            };
            self
        }

        pub fn remove_health(&mut self, health_to_remove: i8) -> &Self {
            self.health -= health_to_remove;
            self
        }

        pub fn add_score(&mut self, score_to_add: i32) -> &Self {
            self.score += score_to_add;
            if self.max_score < self.score {
                self.max_score = self.score;
            }
            self
        }

        pub fn remove_score(&mut self, score_to_remove: i32) -> &Self {
            self.score -= score_to_remove;
            self
        }

        fn render(&self) {
            let Vec2 { x, y } = self.position;
            let Size { width, height } = self.size;
            draw_rectangle(x, y, width, height, BLACK)
        }

        pub fn on_frame(&mut self) {
            let mut new_position = self.position.clone();

            if is_paused() == false {
                if is_key_down(self.controls.move_left) {
                    new_position.x -= DEFAULT_PLAYER_SPEED;
                }
                if is_key_down(self.controls.move_right) {
                    new_position.x += DEFAULT_PLAYER_SPEED;
                }

                if new_position.x < LEFT_MAP_BORDER {
                    new_position.x = LEFT_MAP_BORDER;
                } else if new_position.x + self.size.width > RIGHT_MAP_BORDER {
                    new_position.x = RIGHT_MAP_BORDER - self.size.width;
                }
            }
            self.set_position(new_position);
            self.render();
        }
    }

    #[derive(Clone, Copy)]
    pub enum BallTypes {
        Normal,
        Poisonous,
        Regeneration,
    }

    pub struct Ball {
        pub position: Vec2,
        pub radius: f32,
        pub ball_type: BallTypes,
        pub color: Color,
    }

    impl Ball {
        pub fn new(position: Vec2, ball_type: BallTypes) -> Self {
            Self {
                position,
                radius: DEFAULT_BALL_RADIUS,
                ball_type,
                color: Self::get_color_for_ball_type(&ball_type),
            }
        }

        pub fn gen_ball_position() -> Vec2 {
            Vec2 {
                x: gen_range(
                    LEFT_MAP_BORDER + DEFAULT_BALL_RADIUS + 100.0,
                    RIGHT_MAP_BORDER - DEFAULT_BALL_RADIUS - 100.0,
                ),
                y: DEFAULT_BALL_Y,
            }
        }

        pub fn gen_ball_type() -> BallTypes {
            let random_number = gen_range(0, 100);

            if random_number >= 0 && random_number < 70 {
                BallTypes::Normal
            } else if random_number >= 70 && random_number < 87 {
                BallTypes::Poisonous
            } else {
                BallTypes::Regeneration
            }
        }

        pub fn get_color_for_ball_type(ball_type: &BallTypes) -> Color {
            match ball_type {
                BallTypes::Normal => BLACK,
                BallTypes::Poisonous => RED,
                BallTypes::Regeneration => GREEN,
            }
        }

        pub fn reset(&mut self) -> &Self {
            self.position = Self::gen_ball_position();
            self.ball_type = Self::gen_ball_type();
            self.color = Self::get_color_for_ball_type(&self.ball_type);

            self
        }

        pub fn set_position(&mut self, new_position: Vec2) -> &Self {
            self.position = new_position;

            self
        }

        pub fn check_collision_with_player(&self, player: &Player) -> bool {
            if (player.position.y < (self.position.y + self.radius)
                && (player.position.y + player.size.height) > (self.position.y - self.radius))
                && (player.position.x < (self.position.x + self.radius)
                    && (player.position.x + player.size.width) > (self.position.x - self.radius))
            {
                true
            } else {
                false
            }
        }

        pub fn check_collision_with_ground(&self) -> bool {
            if (self.position.y + self.radius) > GROUND_MAP_BORDER {
                true
            } else {
                false
            }
        }

        fn render(&self) {
            let Vec2 { x, y } = self.position;
            let radius = self.radius;

            draw_circle(x, y, radius, self.color)
        }

        pub fn on_frame(&mut self) {
            let mut new_position = self.position.clone();

            if is_paused() == false {
                new_position.y += 2.0 + (get_difficulty() as f32 / 10.0);
            }

            self.set_position(new_position);
            self.render();
        }
    }
}
