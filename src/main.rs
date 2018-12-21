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

const WND_WIDTH: u32 = 250;
const WND_HEIGHT: u32 = 300;
const AREA_WIDTH: f64 = 200.0;
const AREA_HEIGHT: f64 = 300.0;

fn main() {
    let open_gl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Pong", [WND_WIDTH, WND_HEIGHT])
        .opengl(open_gl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    let mut events = Events::new(EventSettings::new().ups_reset(0));

    let mut world = World::new();
    world.add_resource(GameArea {
        width: AREA_WIDTH,
        height: AREA_HEIGHT,
    });
    let mut dispatcher = DispatcherBuilder::new()
        .with(InputUpdate, "input_update", &[])
        .with(InputApply, "input_apply", &["input_update"])
        .with(Movement, "movement", &["input_apply"])
        .with(OutOfBound, "oob", &["movement"])
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
        .with(Position::new(0.5, 0.5))
        .with(Velocity {
            direction: Vector::new(random::<f64>() * 2.0 - 1.0, random::<f64>() * 10.0 - 5.0)
                .normalize(),
            speed: BALL_DEFAULT_SPEED,
            max_speed: 2.0,
        })
        .with(Ball { size: 0.05 })
        .build();

    // top paddle
    world
        .create_entity()
        .with(Position::new(0.5, 0.975))
        .with(Velocity::new(Default::default(), 0.55))
        .with(Paddle {
            bound: Rectangle::new(Vector::new(-0.125, -0.025), Vector::new(0.125, 0.025)),
        })
        .with(Input::new(Key::Q, Key::D))
        .with(Score::new(|v| v.y < 0.0, Vector::new(1.01, 0.95)))
        .build();

    // bottom paddle
    world
        .create_entity()
        .with(Position::new(0.5, 0.025))
        .with(Velocity::new(Default::default(), 0.55))
        .with(Paddle {
            bound: Rectangle::new(Vector::new(-0.125, -0.025), Vector::new(0.125, 0.025)),
        })
        .with(Input::new(Key::Left, Key::Right))
        .with(Score::new(|v| v.y > 1.0, Vector::new(1.01, 0.05)))
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
