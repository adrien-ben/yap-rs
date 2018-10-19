use game::entities::*;
use game::input::*;
use graphics::{Context, Graphics, text, Transformed};
use graphics::character::CharacterCache;
use math::*;
use piston::input::{Button, Key, UpdateArgs};

mod input;
mod entities;

const TEXT_COLOR: [f32; 4] = [0.8; 4];
const TEXT_SIZE: u32 = 18;
const DEFAULT_SPEED: f64 = 300.0;

#[derive(Eq, PartialEq)]
enum State {
    Idle,
    Running,
}

pub struct Game {
    state: State,
    area: Rectangle,
    ball: Ball,
    bottom_player: Player,
    top_player: Player,
}

impl Game {
    pub fn new(area_width: u32, area_height: u32) -> Self {
        let area = Rectangle {
            bottom_left: Vector { x: 0.0, y: 0.0 },
            top_right: Vector { x: area_width as f64, y: area_height as f64 },
        };

        let bottom_player = Player::new(
            Paddle::new(
                Vector {
                    x: area.width() * 0.5,
                    y: 15.0,
                },
                Rectangle::new(Vector { x: -50.0, y: -15.0 }, Vector { x: 50.0, y: 15.0 }),
            ),
            Input::new(Key::Right, Key::Left),
        );

        let top_player = Player::new(
            Paddle::new(
                Vector {
                    x: area.width() * 0.5,
                    y: area.height() - 15.0,
                },
                Rectangle::new(Vector { x: -50.0, y: -15.0 }, Vector { x: 50.0, y: 15.0 }),
            ),
            Input::new(Key::D, Key::Q),
        );

        let ball = Ball::new(area.center(), DEFAULT_SPEED, 20.0);
        Self {
            state: State::Idle,
            area,
            ball,
            bottom_player,
            top_player,
        }
    }

    pub fn press(&mut self, button: Button) {
        self.check_game_start(button);
        self.bottom_player.input.press(button);
        self.top_player.input.press(button);
    }

    fn check_game_start(&mut self, button: Button) {
        match button {
            Button::Keyboard(key) if key == Key::Space && self.state == State::Idle => {
                self.state = State::Running;
            }
            _ => {}
        }
    }

    pub fn release(&mut self, button: Button) {
        self.bottom_player.input.release(button);
        self.top_player.input.release(button);
    }

    pub fn update(&mut self, args: UpdateArgs) {
        if self.state == State::Idle {} else {
            let dt = args.dt;
            self.bottom_player.paddle.update(self.bottom_player.input, dt);
            self.top_player.paddle.update(self.top_player.input, dt);
            self.ball.update(dt);
            Self::handle_collisions(self);
            Self::check_if_scored_and_update(self);
        }
    }

    fn handle_collisions(&mut self) {
        let bottom_paddle_bound = self.bottom_player.paddle.world_space_bound();
        let top_paddle_bound = self.top_player.paddle.world_space_bound();
        let ball_bound = self.ball.world_space_bound();

        if check_collision(top_paddle_bound, ball_bound)
            || check_collision(bottom_paddle_bound, ball_bound)
            {
                self.ball.on_hit();
            }

        if ball_bound.center.x - ball_bound.radius < self.area.bottom_left.x
            || ball_bound.center.x + ball_bound.radius > self.area.top_right.x
            {
                self.ball.direction.x *= -1.0;
            }
    }

    fn check_if_scored_and_update(&mut self) {
        if self.ball.position.y < self.area.bottom_left.y {
            self.top_player.inc_score();
            self.on_score();
        } else if self.ball.position.y > self.area.top_right.y {
            self.bottom_player.inc_score();
            self.on_score();
        }
    }

    fn on_score(&mut self) {
        self.state = State::Idle;
        self.ball.reset(self.area.center(), DEFAULT_SPEED);
        self.bottom_player.paddle.position.x = self.area.center().x;
        self.top_player.paddle.position.x = self.area.center().x;
    }

    pub fn draw<G: Graphics>(&self, context: &Context, graphics: &mut G) {
        self.top_player.paddle.draw(self.area, context, graphics);
        self.bottom_player.paddle.draw(self.area, context, graphics);
        self.ball.draw(self.area, context, graphics);
    }

    pub fn draw_ui<G, C>(&self, context: &Context, graphics: &mut G, font_cache: &mut C)
        where G: Graphics<Texture=<C>::Texture>,
              C: CharacterCache
    {
        let top_score_trans = context.transform.trans(2.0, TEXT_SIZE as f64);
        text(TEXT_COLOR, TEXT_SIZE, &format!("Score: {}", self.top_player.score), font_cache, top_score_trans, graphics)
            .unwrap_or_else(|_| {});

        let top_score_trans = context.transform.trans(2.0, self.area.top_right.y - 2.0);
        text(TEXT_COLOR, TEXT_SIZE, &format!("Score: {}", self.bottom_player.score), font_cache, top_score_trans, graphics)
            .unwrap_or_else(|_| {});

        if self.state == State::Idle {
            let start_hint_trans = context.transform.trans(2.0, (self.area.top_right.y + TEXT_SIZE as f64) * 0.5);
            text(TEXT_COLOR, TEXT_SIZE, "Space to start", font_cache, start_hint_trans, graphics)
                .unwrap_or_else(|_| {});
        }
    }
}
