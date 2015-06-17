#![feature(associated_consts,core,slice_patterns)]

extern crate core;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

pub mod data;

use piston::window::WindowSettings;
use piston::event::{self,Events,PressEvent,RenderEvent,UpdateEvent};
use piston::input::{Button, Key};
use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{ GlGraphics, OpenGL };

use data::{colors,map};
use data::map::cell::ShapeCell;
use data::map::Map;
use data::shapes::tetrimino::{BLOCK_COUNT,Shape};
use data::gamestate::GameState;

struct App<Rng>{
    gl: GlGraphics,
    tetris: GameState<Rng>,
}

impl<Rng: rand::Rng> App<Rng>{
    fn render(&mut self, args: &event::RenderArgs){
        //Unit square
        const BLOCK_PIXEL_SIZE: f64 = 24.0;
        let square = graphics::rectangle::square(0.0,0.0,BLOCK_PIXEL_SIZE);

        //Draw in the current viewport
        let &mut App{ref mut gl,ref mut tetris} = self;
        gl.draw(args.viewport(),|context,gl|{
            //Clear screen
            graphics::clear(colors::BLACK,gl);

            //Draw map
            graphics::rectangle(colors::LIGHT_BLACK,[0.0,0.0,tetris.map.width() as f64 * BLOCK_PIXEL_SIZE,tetris.map.height() as f64 * BLOCK_PIXEL_SIZE],context.transform,gl);
            for (x,y,ShapeCell(cell)) in tetris.map.cells_positioned(){
                if let Some(cell) = cell{
                    let transform = context.transform.trans(x as f64 * BLOCK_PIXEL_SIZE,y as f64 * BLOCK_PIXEL_SIZE);
                    graphics::rectangle(
                        match cell{
                            Shape::I => colors::blocks::RED,
                            Shape::L => colors::blocks::MAGENTA,
                            Shape::O => colors::blocks::BLUE,
                            Shape::J => colors::blocks::ORANGE,
                            Shape::T => colors::blocks::OLIVE,
                            Shape::S => colors::blocks::LIME,
                            Shape::Z => colors::blocks::CYAN,
                        },
                        square,
                        transform,
                        gl
                    );
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

            //Pause overlay
            if tetris.paused{
                let [w,h] = context.get_view_size();
                graphics::rectangle([0.0,0.0,0.0,0.5],[0.0,0.0,w,h],context.transform,gl);
            }
        });
    }

    fn update(&mut self, args: &event::UpdateArgs){
        self.tetris.update(args);
    }

    fn on_key_press(&mut self, key: Key){
        if self.tetris.paused{match key{
            Key::Return => {self.tetris.paused = false},
            _ => {},
        }}else{match key{
            Key::Right  => {self.tetris.move_block( 1, 0);},
            Key::Left   => {self.tetris.move_block(-1, 0);},
            Key::Down   => {self.tetris.time_count = if self.tetris.move_block( 0, 1){0.0}else{self.tetris.block_move_frequency};},
            Key::Up     => {self.tetris.rotate_and_resolve();},
            Key::X      => {self.tetris.block.previous_rotation();},//TODO: No resolve for previous rotation?
            Key::Z      => {self.tetris.rotate_and_resolve();},
            Key::R      => {self.tetris.map.clear();},
            Key::D1     => {self.tetris.block.set_shape(Shape::I);},
            Key::D2     => {self.tetris.block.set_shape(Shape::L);},
            Key::D3     => {self.tetris.block.set_shape(Shape::O);},
            Key::D4     => {self.tetris.block.set_shape(Shape::J);},
            Key::D5     => {self.tetris.block.set_shape(Shape::T);},
            Key::D6     => {self.tetris.block.set_shape(Shape::S);},
            Key::D7     => {self.tetris.block.set_shape(Shape::Z);},
            Key::Home   => {self.tetris.block_y = 0;},
            Key::Return => {self.tetris.paused = true},
            //Test
            Key::T => {
                //self.tetris.map.
            },
            _ => {},
        }}
    }
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
