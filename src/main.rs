use crate::components::*;
use crate::resources::{DeltaTime, GameArea, GameState};
use crate::systems::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use pong::game::*;
use pong::math::{Rectangle, Vector};
use rand::random;
use specs::prelude::*;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 600;

fn main() {
    let open_gl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Pong", [WIDTH, HEIGHT])
        .opengl(open_gl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    let mut events = Events::new(EventSettings::new().ups_reset(0));

    let mut world = World::new();
    world.add_resource(GameArea {
        width: WIDTH as f64,
        height: HEIGHT as f64,
    });
    let mut dispatcher = DispatcherBuilder::new()
        .with(InputUpdate, "input_update", &[])
        .with(InputApply, "input_apply", &["input_update"])
        .with(OutOfBound, "oob", &["input_apply"])
        .with(Movement, "movement", &["oob"])
        .with(CollisionDetection, "collision_detection", &["movement"])
        .with(
            CollisionResolution,
            "collision_resolution",
            &["collision_detection"],
        )
        .with(ScoreComputer, "score_computer", &[])
        .with_thread_local(Render {
            gl: GlGraphics::new(open_gl),
            glyphs: GlyphCache::new("assets/arial.ttf", (), TextureSettings::new()).unwrap(),
        })
        .build();
    dispatcher.setup(&mut world.res);

    // ball
    world
        .create_entity()
        .with(Position::new(WIDTH as f64 * 0.5, HEIGHT as f64 * 0.5))
        .with(Velocity {
            direction: Vector::new(random::<f64>() * 2.0 - 1.0, random::<f64>() * 10.0 - 5.0)
                .normalize(),
            speed: BALL_DEFAULT_SPEED,
            max_speed: 1200.0,
        })
        .with(Ball { size: 20.0 })
        .build();

    // top paddle
    world
        .create_entity()
        .with(Position::new(WIDTH as f64 * 0.5, HEIGHT as f64 - 15.0))
        .with(Velocity::new(Default::default(), 220.0))
        .with(Paddle {
            bound: Rectangle::new(Vector::new(-50.0, -15.0), Vector::new(50.0, 15.0)),
        })
        .with(Input::new(Key::Q, Key::D))
        .with(Score::new(|v| v.y < 0.0, Vector::new(2.0, 280.0)))
        .build();

    // bottom paddle
    world
        .create_entity()
        .with(Position::new(WIDTH as f64 * 0.5, 15.0))
        .with(Velocity::new(Default::default(), 220.0))
        .with(Paddle {
            bound: Rectangle::new(Vector::new(-50.0, -15.0), Vector::new(50.0, 15.0)),
        })
        .with(Input::new(Key::Left, Key::Right))
        .with(Score::new(|v| v.y > HEIGHT as f64, Vector::new(2.0, 320.0)))
        .build();

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.press_args() {
            *world.write_resource::<crate::resources::PressEvent>() =
                crate::resources::PressEvent(Some(args));
        }

        if let Some(args) = event.release_args() {
            match args {
                Button::Keyboard(Key::Space) => {
                    *world.write_resource::<GameState>() = GameState(State::Running);
                }
                _ => {
                    *world.write_resource::<crate::resources::ReleaseEvent>() =
                        crate::resources::ReleaseEvent(Some(args));
                }
            }
        }

        if let Some(args) = event.render_args() {
            *world.write_resource::<crate::resources::RenderEvent>() =
                crate::resources::RenderEvent(Some(args));
        }

        if let Some(args) = event.update_args() {
            *world.write_resource::<DeltaTime>() = DeltaTime(args.dt);
            dispatcher.dispatch(&mut world.res);
            world.maintain();
        }
    }
}
