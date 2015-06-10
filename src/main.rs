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
use data::shapes::tetrimino::BLOCK_COUNT;
use data::gamestate::GameState;

struct App<Rng>{
    gl: GlGraphics,
    tetris: GameState<Rng>,
}

impl<Rng: rand::Rng> App<Rng>{
    fn render(&mut self, args: &event::RenderArgs){
        //Unit square
        const BLOCK_PIXEL_SIZE: f64 = 16.0;
        let square = graphics::rectangle::square(0.0,0.0,BLOCK_PIXEL_SIZE);

        //Draw in the current viewport
        let &mut App{ref mut gl,ref mut tetris} = self;
        gl.draw(args.viewport(),|context,gl|{
            //Clear screen
            graphics::clear(colors::BLACK,gl);

            //Draw map
            for i in 0..map::WIDTH{
                for j in 0..map::HEIGHT{
                    if tetris.map.position(i as map::PosAxis,j as map::PosAxis){
                        let transform = context.transform.trans(i as f64 * BLOCK_PIXEL_SIZE, j as f64 * BLOCK_PIXEL_SIZE);
                        graphics::rectangle(colors::DARK_WHITE,square,transform,gl);
                    }
                }
            }

            //Draw current block(s)
            for i in 0..BLOCK_COUNT{
                for j in 0..BLOCK_COUNT{
                    if tetris.block.get(i as u8, j as u8){
                        let transform = context.transform.trans((i as map::PosAxis + tetris.block_x) as f64 * BLOCK_PIXEL_SIZE, (j as map::PosAxis + tetris.block_y) as f64 * BLOCK_PIXEL_SIZE);
                        graphics::rectangle(colors::WHITE,square,transform,gl);
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &event::UpdateArgs){
        self.tetris.update(args);
    }

    fn on_key_press(&mut self, key: Key){match key{
        Key::Right => {self.tetris.move_block( 1, 0);},
        Key::Left  => {self.tetris.move_block(-1, 0);},
        Key::Down  => {self.tetris.move_block( 0, 1);},
        Key::Up    => {self.tetris.block.next_rotation();},
        Key::X     => {self.tetris.block.next_rotation();},
        Key::Z     => {self.tetris.block.previous_rotation();},
        _ => {},
    }}
}

fn main(){
    //Define the OpenGL version to be used
    let opengl = OpenGL::_3_2;

    //Create a window.
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "Polyminos Falling",
            [800, 600]
        )
        .exit_on_esc(true)
    );

    //Create a new application
    let mut app = App{
        gl: GlGraphics::new(opengl),
        tetris: GameState::new(rand::StdRng::new().unwrap()),
    };

    //Run the created application: Listen for events
    for e in window.events(){
        //Render
        if let Some(r) = e.render_args(){
            app.render(&r);
        }

        //Update
        if let Some(u) = e.update_args(){
            app.update(&u);
        }

        //Keyboard event
        if let Some(Button::Keyboard(k)) = e.press_args(){
            app.on_key_press(k);
        }
    }
}
