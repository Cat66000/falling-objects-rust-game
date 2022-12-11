pub mod game {
    pub static mut DIFFICULTY: i32 = 1;
    pub static mut PAUSED: bool = false;

    pub const MAX_HEALTH: i8 = 3;

    pub const DEFAULT_PLAYER_SIZE: Size = Size {
        width: 90.0,
        height: 70.0,
    };
    pub const DEFAULT_PLAYER_Y: f32 = 360.0;
    pub const DEFAULT_PLAYER_HEALTH: i8 = 3;
    pub const DEFAULT_PLAYER_SPEED: f32 = 5.0;

    pub const DEFAULT_BALL_RADIUS: f32 = 30.0;

    use macroquad::prelude::*;

    #[derive(Clone, Copy)]
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

            if unsafe { PAUSED } == false {
                if is_key_down(self.controls.move_left) {
                    new_position.x -= DEFAULT_PLAYER_SPEED;
                }
                if is_key_down(self.controls.move_right) {
                    new_position.x += DEFAULT_PLAYER_SPEED;
                }
            }
            self.set_position(new_position);
            self.render();
        }
    }

    pub enum BallTypes {
        Normal,
        Poisonous,
        Regeneration,
    }

    pub struct Ball {
        pub position: Vec2,
        pub radius: f32,
        pub ball_type: BallTypes,
        pub index: usize,
    }

    impl Ball {
        pub fn new(position: Vec2, ball_type: BallTypes, index: usize) -> Self {
            Self {
                position,
                radius: DEFAULT_BALL_RADIUS,
                ball_type,
                index,
            }
        }

        pub fn set_position(&mut self, new_position: Vec2) -> &Self {
            self.position = new_position;

            self
        }

        pub fn check_collision(&self, player: &Player) -> bool {
            if (player.position.y < self.position.y + self.radius
                && player.position.y + player.size.height > self.position.y - self.radius)
                && (player.position.x < self.position.x
                    && (player.position.x + player.size.width) > (self.position.x + self.radius))
            {
                true
            } else {
                false
            }
        }

        fn render(&self) {
            let Vec2 { x, y } = self.position;
            let radius = self.radius;

            let color: Color = match self.ball_type {
                BallTypes::Normal => BLACK,
                BallTypes::Poisonous => RED,
                BallTypes::Regeneration => GREEN,
            };

            draw_circle(x, y, radius, color)
        }

        pub fn on_frame(&mut self) {
            let mut new_position = self.position.clone();
            
            if unsafe { PAUSED } == false {
                new_position.y += 3.0 * (unsafe { DIFFICULTY as f32 } / 2.0);
            }

            self.set_position(new_position);
            self.render();
        }
    }
}
