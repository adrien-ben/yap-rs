use super::{components::*, resources::*, State};
use crate::math::*;
use graphics::{clear, ellipse, rectangle, text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::Button;
use rand::random;
use specs::prelude::*;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const TEXT_COLOR: [f32; 4] = [0.7, 0.7, 0.7, 1.0];
const TEXT_SIZE: u32 = 18;

pub const BALL_DEFAULT_SPEED: f64 = 0.5;

pub struct InputUpdate;

impl<'a> System<'a> for InputUpdate {
    type SystemData = (
        Read<'a, GameState>,
        Write<'a, PressEvent>,
        Write<'a, ReleaseEvent>,
        WriteStorage<'a, Input>,
    );

    fn run(&mut self, (state, mut press_evt, mut release_evt, mut inputs): Self::SystemData) {
        if let GameState(State::Idle) = *state {
            return;
        }

        if let Some(Button::Keyboard(key)) = press_evt.0 {
            for input in (&mut inputs).join() {
                if key == input.key_right {
                    input.right = true;
                }
                if key == input.key_left {
                    input.left = true;
                }
            }
        }
        *press_evt = PressEvent(None);

        if let Some(Button::Keyboard(key)) = release_evt.0 {
            for input in (&mut inputs).join() {
                if key == input.key_right {
                    input.right = false;
                }
                if key == input.key_left {
                    input.left = false;
                }
            }
        }
        *release_evt = ReleaseEvent(None);
    }
}

pub struct InputApply;

impl<'a> System<'a> for InputApply {
    type SystemData = (
        Read<'a, GameState>,
        ReadStorage<'a, Input>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (state, inputs, mut velocities): Self::SystemData) {
        if let GameState(State::Idle) = *state {
            return;
        }

        for (input, velocity) in (&inputs, &mut velocities).join() {
            if input.right {
                velocity.direction = Vector::new(1.0, 0.0);
            } else if input.left {
                velocity.direction = Vector::new(-1.0, 0.0);
            } else {
                velocity.direction = Default::default();
            }
        }
    }
}

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        Read<'a, GameState>,
        Read<'a, DeltaTime>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (state, delta, mut positions, velocities): Self::SystemData) {
        if let GameState(State::Idle) = *state {
            return;
        }

        let delta = delta.0;
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.current += velocity.direction * velocity.speed * delta;
        }
    }
}

pub struct OutOfBound;

impl<'a> System<'a> for OutOfBound {
    type SystemData = (
        Read<'a, GameState>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (state, mut positions, balls, paddles, mut velocities): Self::SystemData) {
        if let GameState(State::Idle) = *state {
            return;
        }

        for (position, ball, velocity) in (&mut positions, &balls, &mut velocities).join() {
            if position.current.x - ball.size * 0.5 < 0.0 || position.current.x + ball.size * 0.5 > 1.0 {
                if position.current.x + ball.size * 0.5 < 0.0 {
                    position.current.x = ball.size * 0.5;
                }
                if position.current.x - ball.size * 0.5 > 1.0 {
                    position.current.x = 1.0 - ball.size * 0.5;
                }
                velocity.direction.x *= -1.0;
            }
        }
        for (position, paddle, velocity) in (&mut positions, &paddles, &mut velocities).join() {
            if position.current.x + paddle.bound.top_right.x < 0.0
                || position.current.x + paddle.bound.bottom_left.x > 1.0
            {
                if position.current.x + paddle.bound.top_right.x < 0.0 {
                    position.current.x = -paddle.bound.bottom_left.x;
                }
                if position.current.x + paddle.bound.bottom_left.x > 1.0 {
                    position.current.x = 1.0 - paddle.bound.top_right.x;
                }
                velocity.direction.x *= -1.0;
            }
        }
    }
}

pub struct CollisionDetection;

impl<'a> System<'a> for CollisionDetection {
    type SystemData = (
        Read<'a, GameState>,
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Collision>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
    );

    fn run(
        &mut self,
        (state, entities, positions, mut collisions, balls, paddles): Self::SystemData,
    ) {
        if let GameState(State::Idle) = *state {
            return;
        }

        for (entity, ball_pos, ball) in (&entities, &positions, &balls).join() {
            for (paddle_pos, paddle) in (&positions, &paddles).join() {
                let circle = Circle {
                    center: ball_pos.current,
                    radius: ball.size * 0.5,
                };
                let rectangle = Rectangle::new(
                    paddle_pos.current + paddle.bound.bottom_left,
                    paddle_pos.current + paddle.bound.top_right,
                );
                if check_collision(rectangle, circle) {
                    collisions.insert(entity, Collision).unwrap();
                }
            }
        }
    }
}

pub struct CollisionResolution;

impl<'a> System<'a> for CollisionResolution {
    type SystemData = (
        Read<'a, GameState>,
        Read<'a, DeltaTime>,
        Entities<'a>,
        WriteStorage<'a, Collision>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (state, delta, entities, collisions, mut positions, mut velocities, updater): Self::SystemData,
    ) {
        if let GameState(State::Idle) = *state {
            return;
        }

        for (entity, _, position, velocity) in
            (&entities, &collisions, &mut positions, &mut velocities).join()
        {
            velocity.direction.y *= -1.0;
            velocity.speed = velocity.max_speed.min(velocity.speed * 1.1);
            position.current.y += velocity.direction.y * velocity.speed * delta.0;
            updater.remove::<Collision>(entity);
        }
    }
}

pub struct ScoreComputer;

impl<'a> System<'a> for ScoreComputer {
    type SystemData = (
        Write<'a, GameState>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        WriteStorage<'a, Score>,
    );

    fn run(
        &mut self,
        (mut state, mut positions, mut velocities, balls, mut scores): Self::SystemData,
    ) {
        if let GameState(State::Idle) = *state {
            return;
        }

        let mut scored = false;
        'outter: for (position, _) in (&mut positions, &balls).join() {
            for score in (&mut scores).join() {
                if (score.trigger)(position.current) {
                    *state = GameState(State::Idle);
                    score.current += 1;
                    scored = true;
                    break 'outter;
                }
            }
        }

        if scored {
            for position in (&mut positions).join() {
                position.current = position.default;
            }

            for (velocity, _) in (&mut velocities, &balls).join() {
                let dir = Vector::new(random::<f64>() * 2.0 - 1.0, random::<f64>() * 10.0 - 5.0);
                velocity.direction = dir.normalize();
                velocity.speed = BALL_DEFAULT_SPEED;
            }
        }
    }
}

pub struct Render<'a> {
    pub gl: GlGraphics,
    pub glyphs: GlyphCache<'a>,
}

impl<'a, 'b> System<'a> for Render<'b> {
    type SystemData = (
        ReadExpect<'a, GameArea>,
        Read<'a, GameState>,
        Write<'a, RenderEvent>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Score>,
    );

    fn run(
        &mut self,
        (area, state, mut event, positions, balls, paddles, scores): Self::SystemData,
    ) {
        if let Some(args) = event.0 {
            let glyphs = &mut self.glyphs;

            self.gl.draw(args.viewport(), |context, graphics| {
                clear(BLACK, graphics);

                for (position, ball) in (&positions, &balls).join() {
                    ellipse(
                        WHITE,
                        [
                            -ball.size * 0.5 * area.width,
                            -ball.size * 0.5 * area.height,
                            ball.size * area.width,
                            ball.size * area.width,
                        ],
                        context.transform.trans(
                            position.current.x * area.width,
                            (1.0 - position.current.y) * area.height,
                        ),
                        graphics,
                    );
                }

                for (position, paddle) in (&positions, &paddles).join() {
                    rectangle(
                        WHITE,
                        [
                            paddle.bound.bottom_left.x * area.width,
                            paddle.bound.bottom_left.y * area.height,
                            paddle.bound.width() * area.width,
                            paddle.bound.height() * area.height,
                        ],
                        context.transform.trans(
                            position.current.x * area.width,
                            (1.0 - position.current.y) * area.height,
                        ),
                        graphics,
                    );
                }

                for score in (&scores).join() {
                    let text_transform = context.transform.trans(
                        score.position.x * area.width,
                        (1.0 - score.position.y) * area.height,
                    );
                    text(
                        TEXT_COLOR,
                        TEXT_SIZE,
                        &format!("Score: {}", score.current),
                        glyphs,
                        text_transform,
                        graphics,
                    )
                    .unwrap_or_else(|_| {});
                }

                if let GameState(State::Idle) = *state {
                    let text_transform = context.transform.trans(
                        area.width * 0.5 - 60.0,
                        area.height * 0.5 + TEXT_SIZE as f64 * 0.5,
                    );
                    text(
                        TEXT_COLOR,
                        TEXT_SIZE,
                        "Space to start",
                        glyphs,
                        text_transform,
                        graphics,
                    )
                    .unwrap_or_else(|_| {});
                }
            });
        }
        *event = RenderEvent(None);
    }
}
