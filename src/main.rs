#![feature(associated_consts,collections,core,slice_patterns,vecmap)]

extern crate collections;
extern crate core;
#[macro_use] extern crate enum_primitive;
extern crate graphics;
extern crate num;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
#[cfg(feature = "include_sdl2")]  extern crate sdl2_window;
#[cfg(feature = "include_glfw")]  extern crate glfw_window;
#[cfg(feature = "include_glutin")]extern crate glutin_window;

pub mod controller;
pub mod data;
pub mod gamestate;

use piston::window::WindowSettings;
use piston::event::{self,Events,PressEvent,RenderEvent,UpdateEvent};
use piston::input::{Button,Key};
use graphics::Transformed;
use opengl_graphics::{GlGraphics,OpenGL};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
#[cfg(feature = "include_sdl2")]  use sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")]  use glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")]use glutin_window::GlutinWindow as Window;

use controller::ai;
use data::{cell,colors,player};
use data::grid::{self,Grid};
use data::map::dynamic_map::Map;
use data::shapes::tetrimino::{Shape,RotatedShape};
use data::input::Input;
use gamestate::{GameState, PlayerId};

struct App<Rng>{
    gl: GlGraphics,
    tetris: GameState<Map<cell::ShapeCell>,Rng>,
    input_receiver: Receiver<(Input, PlayerId)>,
    input_sender: Sender<(Input, PlayerId)>,
    connection: Connection,
}

pub enum Connection {
    Server,
    Client(String),
}

impl<Rng: rand::Rng> App<Rng>{
    fn render(&mut self, args: &event::RenderArgs){
        const BLOCK_PIXEL_SIZE: f64 = 24.0;

        fn map_render_pos(map_no: usize) -> (f64,f64){
            (map_no as f64 * 12.0 * BLOCK_PIXEL_SIZE,0.0)
        }

        //Unit square
        let square = graphics::rectangle::square(0.0,0.0,BLOCK_PIXEL_SIZE);

        //Draw in the current viewport
        let &mut App{ref mut gl,ref mut tetris, ..} = self;
        gl.draw(args.viewport(),|context,gl|{
            //Clear screen
            graphics::clear(colors::BLACK,gl);

            //Draw maps
            for (map_id,map) in tetris.maps.iter(){
                let transform = {
                    let (x,y) = map_render_pos(map_id);
                    context.transform.trans(x,y)
                };

                //Background
                graphics::rectangle(colors::LIGHT_BLACK,[0.0,0.0,map.width() as f64 * BLOCK_PIXEL_SIZE,map.height() as f64 * BLOCK_PIXEL_SIZE],transform,gl);

                //Imprinted cells
                for (cell_pos,cell::ShapeCell(cell)) in grid::cells_iter::Iter::new(map){
                    if let Some(cell) = cell{
                        let transform = transform.trans(cell_pos.x as f64 * BLOCK_PIXEL_SIZE,cell_pos.y as f64 * BLOCK_PIXEL_SIZE);
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
            }

            //Draw players
            for (_,player) in tetris.players.iter(){match tetris.maps.get(&(player.map as usize)){
                Some(_) => {
                    let transform = {
                        let (x,y) = map_render_pos(player.map as usize);
                        context.transform.trans(x,y)
                    };

                    //Select color
                    let color = match player.shape.shape(){
                        Shape::I => colors::shapes::LIGHT_RED,
                        Shape::L => colors::shapes::LIGHT_MAGENTA,
                        Shape::O => colors::shapes::LIGHT_BLUE,
                        Shape::J => colors::shapes::LIGHT_ORANGE,
                        Shape::T => colors::shapes::LIGHT_OLIVE,
                        Shape::S => colors::shapes::LIGHT_LIME,
                        Shape::Z => colors::shapes::LIGHT_CYAN,
                    };

                    //Draw current shape(s)
                    for (cell_pos,cell) in grid::cells_iter::Iter::new(&player.shape){
                        if cell{
                            //Normal shape
                            {
                                let transform = transform.trans((cell_pos.x as grid::PosAxis + player.pos.x) as f64 * BLOCK_PIXEL_SIZE, (cell_pos.y as grid::PosAxis + player.pos.y) as f64 * BLOCK_PIXEL_SIZE);
                                graphics::rectangle(color,square,transform,gl);
                            }

                            //Shadow shape
                            if let Some(shadow_pos) = player.shadow_pos{
                                let transform = transform.trans((cell_pos.x as grid::PosAxis + shadow_pos.x) as f64 * BLOCK_PIXEL_SIZE, (cell_pos.y as grid::PosAxis + shadow_pos.y) as f64 * BLOCK_PIXEL_SIZE);
                                let color = [color[0],color[1],color[2],0.3];
                                graphics::rectangle(color,square,transform,gl);
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
        self.handle_input();
        self.tetris.update(args);
    }

    fn handle_input(&mut self){
        match self.input_receiver.try_recv(){
            Ok((input,pid)) => {
                match input{
                    Input::MoveLeft => {self.tetris.with_player_map(pid,|player,map|{
                        gamestate::move_player(player,map,grid::Pos{x: -1, y: 0});
                    });},
                    Input::MoveRight => {self.tetris.with_player_map(pid,|player,map|{
                        gamestate::move_player(player,map,grid::Pos{x: 1, y: 0});
                    });},
                    Input::SlowFall => {self.tetris.with_player_map(pid,|player,map|{
                        player.gravityfall_time_count = if gamestate::move_player(player,map,grid::Pos{x: 0,y: 1}){
                            //reset timer
                            0.0
                        } else {
                            //Set timer and make the player move in the next update step
                            player.settings.gravityfall_frequency
                        };
                    });},
                    Input::FastFall => {self.tetris.with_player_map(pid,|player,map|{
                        player.pos = gamestate::fast_fallen_shape(&player.shape, map, player.pos);
                        player.gravityfall_time_count = player.settings.gravityfall_frequency;
                    });},
                    Input::RotateAntiClockwise => {self.tetris.with_player_map(pid,|player,map|{
                        let shape = player.shape.rotated_anticlockwise();
                        gamestate::transform_resolve_player(player, shape, map);
                    });},
                    Input::RotateClockwise => {self.tetris.with_player_map(pid,|player,map|{
                        let shape = player.shape.rotated_clockwise();
                        gamestate::transform_resolve_player(player, shape, map);
                    });},
                    _ => (),
                }
                self.handle_input()
            }
            Err(_) => ()
        }
    }

    fn on_key_press(&mut self, key: Key){
        if self.tetris.paused{match key{
            Key::Return => {self.tetris.paused = false},
            _ => {},
        }}else{match key{
            Key::Return => {self.tetris.paused = true},

            //Player 0 Tests
            Key::D1     => {self.tetris.with_player(0,|player|{player.shape = RotatedShape::new(Shape::I);});},
            Key::D2     => {self.tetris.with_player(0,|player|{player.shape = RotatedShape::new(Shape::L);});},
            Key::D3     => {self.tetris.with_player(0,|player|{player.shape = RotatedShape::new(Shape::O);});},
            Key::D4     => {self.tetris.with_player(0,|player|{player.shape = RotatedShape::new(Shape::J);});},
            Key::D5     => {self.tetris.with_player(0,|player|{player.shape = RotatedShape::new(Shape::T);});},
            Key::D6     => {self.tetris.with_player(0,|player|{player.shape = RotatedShape::new(Shape::S);});},
            Key::D7     => {self.tetris.with_player(0,|player|{player.shape = RotatedShape::new(Shape::Z);});},
            Key::R      => {
                match self.tetris.players.get(&(0 as usize)).map(|player| player.map){
                    Some(map_id) => {self.tetris.reset_map(map_id);},
                    None => ()
                };
            },
            Key::Home   => {self.tetris.with_player(0,|player|{player.pos.y = 0;});},

            //Player 0
            Key::Left   => {self.input_sender.send((Input::MoveLeft,           0)).unwrap();},
            Key::Right  => {self.input_sender.send((Input::MoveRight,          0)).unwrap();},
            Key::Down   => {self.input_sender.send((Input::SlowFall,           0)).unwrap();},
            Key::End    => {self.input_sender.send((Input::FastFall,           0)).unwrap();},
            Key::X      => {self.input_sender.send((Input::RotateAntiClockwise,0)).unwrap();},
            Key::Z      => {self.input_sender.send((Input::RotateClockwise,    0)).unwrap();},
           
            //Player 1
            Key::NumPad4 => {self.input_sender.send((Input::MoveLeft,           1)).unwrap();},
            Key::NumPad6 => {self.input_sender.send((Input::MoveRight,          1)).unwrap();},
            Key::NumPad5 => {self.input_sender.send((Input::SlowFall,           1)).unwrap();},
            Key::NumPad2 => {self.input_sender.send((Input::FastFall,           1)).unwrap();},
            Key::NumPad1 => {self.input_sender.send((Input::RotateAntiClockwise,1)).unwrap();},
            Key::NumPad0 => {self.input_sender.send((Input::RotateClockwise,    1)).unwrap();},

            //Other keys
            _ => ()
        }}
    }
}

fn main(){
    use std::env;
    //Define the OpenGL version to be used
    let opengl = OpenGL::_3_2;

    //Create a window.
    let window = Window::new(
        WindowSettings::new(
            "Polyminos Falling",
            [800, 600]
        )
        .exit_on_esc(true)
        .opengl(opengl)
    );
    
    let (input_sender, input_receiver) = mpsc::channel();
    let args: Vec<_> = env::args().collect();

    //Create a new application
    let mut app = App{
        gl: GlGraphics::new(opengl),
        tetris: GameState::new(rand::StdRng::new().unwrap()),
        input_receiver: input_receiver,
        input_sender: input_sender,
        connection: if args.len() > 1 {
            Connection::Client(args[1].clone())
        } else {Connection::Server},
    };

    //Create map
    app.tetris.maps.insert(0,Map::new(10,20));
    app.tetris.maps.insert(1,Map::new(10,20));

    //Create player 0
    app.tetris.add_player(0,player::Settings{
        gravityfall_frequency: 1.0,
        slowfall_delay       : 1.0,
        slowfall_frequency   : 1.0,
        move_delay           : 1.0,
        move_frequency       : 1.0,
        fastfall_shadow      : true,
    });

    //Create player 1
    /*app.tetris.add_player(1,player::Settings{
        gravityfall_frequency: 0.5,
        slowfall_delay       : 1.0,
        slowfall_frequency   : 1.0,
        move_delay           : 1.0,
        move_frequency       : 1.0,
        fastfall_shadow      : true,
    });*/

    //Create player 2
    let player2 = app.tetris.add_player(1,player::Settings{
        gravityfall_frequency: 1.0,
        slowfall_delay       : 1.0,
        slowfall_frequency   : 1.0,
        move_delay           : 1.0,
        move_frequency       : 1.0,
        fastfall_shadow: false,
    }).unwrap();
    app.tetris.controllers.insert(player2 as usize,Box::new(ai::bruteforce::Controller::default()));

    //Run the created application: Listen for events
    for e in window.events(){
        //Player inflicted input: Keyboard events
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
