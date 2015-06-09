#![feature(associated_consts,core)]

extern crate core;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod data;

use piston::window::WindowSettings;
use piston::event::{self,Events,PressEvent,RenderEvent,UpdateEvent};
use piston::input::{Button, Key};
use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{ GlGraphics, OpenGL };

use data::{colors,map};
use data::shapes::tetrimino::{Shape, BLOCK_COUNT};
use data::gamestate::GameState;

struct App<Rng>{
    gl: GlGraphics,
    tetris : GameState<Rng>,
}

impl<Rng: rand::Rng> App<Rng>{
    fn render(&mut self, args: &event::RenderArgs) {
        let square = graphics::rectangle::square(0.0, 0.0, 16.0);
        let &mut App{ref mut gl,ref mut tetris} = self;

        gl.draw(args.viewport(), |c, g| {
            graphics::clear(colors::BLACK, g);

            for i in 0..map::WIDTH {
                for j in 0..map::HEIGHT {
                    if tetris.map.position(i as map::PosAxis,j as map::PosAxis) {
                        let transform = c.transform.trans(i as f64 * 16.0, j as f64 * 16.0);
                        graphics::rectangle(colors::WHITE, square, transform, g);
                    }
                }
            }

            for i in 0..BLOCK_COUNT {
                for j in 0..BLOCK_COUNT {
                    if tetris.block.get(i as u8, j as u8) {
                        let transform = c.transform.trans((i as map::PosAxis + tetris.block_x) as f64 * 16.0, (j as map::PosAxis + tetris.block_y) as f64 * 16.0);
                        graphics::rectangle(colors::WHITE, square, transform, g);
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &event::UpdateArgs) {
        self.tetris.update(args);
    }

    fn on_key_press(&mut self, key : Key) {
        match key {
            Key::Right => {self.tetris.move_block(1, 0);},
            Key::Left  => {self.tetris.move_block(-1, 0);},
            Key::Down  => {self.tetris.move_block(0, 1);},
            Key::Up    => {self.tetris.block.next_rotation();},
            Key::X     => {self.tetris.block.next_rotation();},
            Key::Z     => {self.tetris.block.previous_rotation();},
            _ => {},
        }
    }
}

fn main() {
    let opengl = OpenGL::_3_2;

    //Create an Glutin window.
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "Polyminos Falling",
            [800, 600]
        )
        .exit_on_esc(true)
    );

    //Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        tetris: GameState::new(rand::StdRng::new().unwrap()),
    };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(k)) = e.press_args() {
            app.on_key_press(k);
        }
    }
}
