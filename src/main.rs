#![feature(associated_consts,collections,core,slice_patterns)]

extern crate collections;
extern crate core;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

pub mod data;

use piston::window::WindowSettings;
use piston::event::{self,Events,PressEvent,RenderEvent,UpdateEvent};
use piston::input::{Button,Key};
use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{GlGraphics,OpenGL};

use data::{colors,map};
use data::map::cell::ShapeCell;
use data::map::Map;
use data::shapes::tetrimino::{BLOCK_COUNT,Shape};
use data::gamestate::{self,GameState};

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

            for (_,player) in tetris.players.iter(){match tetris.maps.get(&(player.map as usize)){
                Some(map) => {
                    //Draw map
                    graphics::rectangle(colors::LIGHT_BLACK,[0.0,0.0,map.width() as f64 * BLOCK_PIXEL_SIZE,map.height() as f64 * BLOCK_PIXEL_SIZE],context.transform,gl);
                    for (x,y,ShapeCell(cell)) in map.cells_positioned(){
                        if let Some(cell) = cell{
                            let transform = context.transform.trans(x as f64 * BLOCK_PIXEL_SIZE,y as f64 * BLOCK_PIXEL_SIZE);
                            graphics::rectangle(
                                match cell{
                                    Shape::I => colors::shapes::RED,
                                    Shape::L => colors::shapes::MAGENTA,
                                    Shape::O => colors::shapes::BLUE,
                                    Shape::J => colors::shapes::ORANGE,
                                    Shape::T => colors::shapes::OLIVE,
                                    Shape::S => colors::shapes::LIME,
                                    Shape::Z => colors::shapes::CYAN,
                                },
                                square,
                                transform,
                                gl
                            );
                        }
                    }

                    //Draw current shape(s)
                    for i in 0..BLOCK_COUNT{
                        for j in 0..BLOCK_COUNT{
                            if player.shape.get(i as u8, j as u8){
                                let transform = context.transform.trans((i as map::PosAxis + player.x) as f64 * BLOCK_PIXEL_SIZE, (j as map::PosAxis + player.y) as f64 * BLOCK_PIXEL_SIZE);
                                graphics::rectangle(colors::WHITE,square,transform,gl);
                            }
                        }
                    }
                },
                None => ()
            }}

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
            Key::Return => {self.tetris.paused = true},
            key => if let Some(player) = self.tetris.players.get_mut(&0){match key{
                Key::Left   => if let Some(map) = self.tetris.maps.get_mut(&(player.map as usize)){gamestate::move_player(player,map,-1,0);},
                Key::Right  => if let Some(map) = self.tetris.maps.get_mut(&(player.map as usize)){gamestate::move_player(player,map, 1,0);},
                Key::Down   => if let Some(map) = self.tetris.maps.get_mut(&(player.map as usize)){
                    player.move_time_count = if gamestate::move_player(player,map,0,1){
                        //Reset timer
                        0.0
                    }else{
                        //Set timer and make the player move in the update step
                        player.move_frequency
                };},
                Key::Up     => if let Some(map) = self.tetris.maps.get_mut(&(player.map as usize)){gamestate::rotate_and_resolve_player(player,map);},
                Key::X      => if let Some(map) = self.tetris.maps.get_mut(&(player.map as usize)){gamestate::rotate_and_resolve_player(player,map);},
                Key::Z      => {player.shape.previous_rotation();},//TODO: No resolve for previous rotation?
                Key::R      => if let Some(map) = self.tetris.maps.get_mut(&(player.map as usize)){map.clear();},
                Key::D1     => {player.shape.set_shape(Shape::I);},
                Key::D2     => {player.shape.set_shape(Shape::L);},
                Key::D3     => {player.shape.set_shape(Shape::O);},
                Key::D4     => {player.shape.set_shape(Shape::J);},
                Key::D5     => {player.shape.set_shape(Shape::T);},
                Key::D6     => {player.shape.set_shape(Shape::S);},
                Key::D7     => {player.shape.set_shape(Shape::Z);},
                Key::Home   => {player.y = 0;},
                _           => {}
            }},
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
        //Keyboard event
        if let Some(Button::Keyboard(k)) = e.press_args(){
            app.on_key_press(k);
        }

        //Update
        if let Some(u) = e.update_args(){
            app.update(&u);
        }

        //Render
        if let Some(r) = e.render_args(){
            app.render(&r);
        }
    }
}
