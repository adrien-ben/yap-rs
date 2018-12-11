use glutin_window::GlutinWindow as Window;
use graphics::clear;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, Events, EventSettings};
use piston::input::{PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use pong::game::*;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WIDTH: u32 = 400;
const HEIGHT: u32 = 600;

fn main() {
    let open_gl = OpenGL::V3_2;
    let mut window: Window =
        WindowSettings::new("Pong", [WIDTH, HEIGHT])
            .opengl(open_gl)
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();
    let mut gl = GlGraphics::new(open_gl);
    let mut glyphs = GlyphCache::new("assets/arial.ttf", (), TextureSettings::new()).unwrap();
    let mut events = Events::new(EventSettings::new().ups_reset(0));

    let mut game = Game::new(WIDTH, HEIGHT);

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.press_args() {
            game.press(args);
        }

        if let Some(args) = event.release_args() {
            game.release(args);
        }

        if let Some(args) = event.update_args() {
            game.update(args);
        }

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, graphics| {
                clear(BLACK, graphics);
                game.draw(&context, graphics);
                game.draw_ui(&context, graphics, &mut glyphs);
            });
        }
    }
}
