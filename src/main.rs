#![allow(unused_imports)]
#![allow(dead_code)]
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub mod data;
use data::gamestate::GameState;
use data::colors::*;

pub struct App {
    gl: GlGraphics,
    tetris : GameState, 
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let (tetris, gl) = (&self.tetris, &mut self.gl);
        tetris.render(gl, args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.tetris.update(args);
    }
}

fn main() {
    let opengl = OpenGL::_3_2;

    // Create an Glutin window.
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "spinning-square",
            [800, 600]
        )
        .exit_on_esc(true)
    );

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        tetris: GameState::new(),
    };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
