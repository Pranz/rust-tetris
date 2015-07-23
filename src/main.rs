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
pub mod input;
pub mod online;
pub mod render;

use num::traits::FromPrimitive;
use piston::window::WindowSettings;
use piston::event::{self,Events,PressEvent,RenderEvent,UpdateEvent};
use piston::input::{Button,Key};
use opengl_graphics::{GlGraphics,OpenGL};
use std::{net,sync,thread};
use std::net::ToSocketAddrs;
#[cfg(feature = "include_sdl2")]  use sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")]  use glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")]use glutin_window::GlutinWindow as Window;

use controller::ai;
use data::{cell,player};
use data::grid::Grid;
use data::map::dynamic_map::Map;
use data::shapes::tetrimino::{Shape,RotatedShape};
use data::input::Input;
use gamestate::{GameState, PlayerId};

struct App<Rng>{
    gl: GlGraphics,
    tetris: GameState<Map<cell::ShapeCell>,Rng>,
    input_receiver: sync::mpsc::Receiver<(Input, PlayerId)>,
    input_sender: sync::mpsc::Sender<(Input, PlayerId)>,
    connection: online::ConnectionType,
}

impl<Rng: rand::Rng> App<Rng>{
    fn update(&mut self, args: &event::UpdateArgs){
        //Input        
        while let Ok((input,pid)) = self.input_receiver.try_recv(){
            self.tetris.with_player_map(pid,|player,map|{
                input::perform(input,player,map);
            });

            if let online::ConnectionType::Client(ref socket,ref address) = self.connection{if pid==0{
                socket.send_to(&[input as u8],address).unwrap();
            }}
        }

        //Update
        self.tetris.update(args);
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

    let (input_sender, input_receiver) = sync::mpsc::channel();
    let args: Vec<_> = env::args().collect();

    //Create a new application
    let mut app = App{
        gl: GlGraphics::new(opengl),
        tetris: GameState::new(rand::StdRng::new().unwrap()),
        input_receiver: input_receiver,
        input_sender: input_sender.clone(),
        connection: if args.len() > 1{
            match net::UdpSocket::bind((net::Ipv4Addr::new(0,0,0,0),7375)){
                Ok(socket) => {
                    online::ConnectionType::Client(socket,(&*args[1]).to_socket_addrs().unwrap().next().unwrap())
                },
                Err(e) => {
                    println!("Client socket error: {:?}",e);
                    online::ConnectionType::None
                }
            }
        }else{
            match net::UdpSocket::bind((net::Ipv4Addr::new(0,0,0,0),7374)){
                Ok(socket) => {
                    thread::spawn(move ||{
                        let mut buffer = [0];
                        loop{
                            socket.recv_from(&mut buffer).unwrap();
                            match Input::from_u8(buffer[0]){
                                Some(input) => {
                                    input_sender.send((input,1));
                                },
                                None => ()
                            }
                        }
                    });
                    online::ConnectionType::Server
                },
                Err(e) => {
                    println!("Server socket error: {:?}",e);
                    online::ConnectionType::None
                }
            }
        },
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
    app.tetris.add_player(1,player::Settings{
        gravityfall_frequency: 1.0,
        slowfall_delay       : 1.0,
        slowfall_frequency   : 1.0,
        move_delay           : 1.0,
        move_frequency       : 1.0,
        fastfall_shadow      : true,
    });

    //Create player 2
    /*let player2 = app.tetris.add_player(1,player::Settings{
        gravityfall_frequency: 1.0,
        slowfall_delay       : 1.0,
        slowfall_frequency   : 1.0,
        move_delay           : 1.0,
        move_frequency       : 1.0,
        fastfall_shadow: false,
    }).unwrap();
    app.tetris.controllers.insert(player2 as usize,Box::new(ai::bruteforce::Controller::default()));*/

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
            render::default::gamestate(&mut app.tetris,&mut app.gl,&r);
        }
    }
}
