#![feature(associated_consts,collections,core,ip_addr,lookup_host,optin_builtin_traits,plugin,raw,slice_patterns,str_split_at,vecmap)]

#![plugin(docopt_macros)]
extern crate collections;
extern crate core;
extern crate docopt;
extern crate endian_type;
#[macro_use] extern crate enum_primitive;
extern crate graphics;
extern crate num;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate rustc_serialize;
#[cfg(feature = "include_sdl2")]  extern crate sdl2_window;
#[cfg(feature = "include_glfw")]  extern crate glfw_window;
#[cfg(feature = "include_glutin")]extern crate glutin_window;

mod command_arg;
pub mod controller;
pub mod data;
pub mod gamestate;
pub mod game;
pub mod input;
pub mod online;
pub mod render;
pub mod tmp_ptr;

use controller::Controller;
use endian_type::types::*;
use num::FromPrimitive;
use piston::window::WindowSettings;
use piston::event::{self,Events,PressEvent,RenderEvent,UpdateEvent};
use piston::input::{Button,Key};
use opengl_graphics::{GlGraphics,OpenGL};
use std::{net,sync,thread};
#[cfg(feature = "include_sdl2")]  use sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")]  use glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")]use glutin_window::GlutinWindow as Window;

use controller::ai;
use data::{cell,player};
use data::grid::Grid;
use data::map::dynamic_map::Map;
use data::player::Player;
use data::shapes::tetrimino::{Shape,RotatedShape};
use data::input::Input;
use gamestate::{GameState,MapId,PlayerId};
use game::event::Event;
use tmp_ptr::TmpPtr;

struct App{
    gl: GlGraphics,
    tetris: GameState<Map<cell::ShapeCell>,rand::StdRng>,
    controllers: Vec<Box<Controller<Map<cell::ShapeCell>,Event<(PlayerId,TmpPtr<Player>),(MapId,TmpPtr<Map<cell::ShapeCell>>)>>>>,
    input_receiver: sync::mpsc::Receiver<(Input, PlayerId)>,
    connection: online::ConnectionType,
    paused: bool,
}

impl App{
    fn update(&mut self, args: &event::UpdateArgs){
        //Controllers
        if !self.paused{
            for mut controller in self.controllers.iter_mut(){
                controller.update(args,&self.tetris.players,&self.tetris.maps);
            }
        }

        //Input
        while let Ok((input,pid)) = self.input_receiver.try_recv(){
            if let Some(player) = self.tetris.players.get_mut(&(pid as usize)){
                if let Some(map) = self.tetris.maps.get_mut(&(player.map as usize)){
                    input::perform(input,player,map);
                }
            }

            if let online::ConnectionType::Client(ref socket,ref address) = self.connection{if pid==0{
                socket.send_to(online::client::packet::PlayerInput{
                    connection_id: u32_le::from(0),
                    player_network_id: u32_le::from(0),
                    input: input as u8
                }.into_packet().as_bytes(),address).unwrap();
            }}
        }

        //Update
        if !self.paused{
            let &mut App{tetris: ref mut game,controllers: ref mut cs,..} = self;
            game.update(args,&mut |e| for c in cs.iter_mut(){c.event(e);});
        }
    }

    fn on_key_press(&mut self,key: Key,input_sender: &sync::mpsc::Sender<(Input,PlayerId)>){
        if self.paused{match key{
            Key::Return => {self.paused = false},
            _ => {},
        }}else{match key{
            Key::Return => {self.paused = true},

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
                    Some(map_id) => {
                        let &mut App{tetris: ref mut game,controllers: ref mut cs,..} = self;
                        game.reset_map(map_id,&mut |e| for c in cs.iter_mut(){c.event(e);});
                    },
                    None => ()
                };
            },
            Key::Home   => {self.tetris.with_player(0,|player|{player.pos.y = 0;});},

            //Player 0
            Key::Left   => {input_sender.send((Input::MoveLeft,           0)).unwrap();},
            Key::Right  => {input_sender.send((Input::MoveRight,          0)).unwrap();},
            Key::Down   => {input_sender.send((Input::SlowFall,           0)).unwrap();},
            Key::End    => {input_sender.send((Input::FastFall,           0)).unwrap();},
            Key::X      => {input_sender.send((Input::RotateAntiClockwise,0)).unwrap();},
            Key::Z      => {input_sender.send((Input::RotateClockwise,    0)).unwrap();},

            //Player 1
            Key::NumPad4 => {input_sender.send((Input::MoveLeft,           1)).unwrap();},
            Key::NumPad6 => {input_sender.send((Input::MoveRight,          1)).unwrap();},
            Key::NumPad5 => {input_sender.send((Input::SlowFall,           1)).unwrap();},
            Key::NumPad2 => {input_sender.send((Input::FastFall,           1)).unwrap();},
            Key::NumPad1 => {input_sender.send((Input::RotateAntiClockwise,1)).unwrap();},
            Key::NumPad0 => {input_sender.send((Input::RotateClockwise,    1)).unwrap();},

            //Other keys
            _ => ()
        }}
    }
}

docopt!(Args derive Debug,"
Usage: tetr [options]
       tetr --help

Options:
  -h, --help           Show this message
  -v, --version        Show version
  --online=CONNECTION  Available modes: none, server, client [default: none]
  --host=ADDR          Network address used for the online connection [default: 0.0.0.0]
  --port=N             Network port used for the online connection [default: 7374]
  --window-size=SIZE   Window size [default: 800x600]
  --window-mode=MODE   Available modes: window, fullscreen [default: window]
",
    flag_online     : command_arg::OnlineConnection,
    flag_host       : command_arg::Host,
    flag_port       : command_arg::Port,
    flag_window_size: command_arg::WindowSize,
    flag_window_mode: command_arg::WindowMode
);

fn main(){
    let args: Args = match Args::docopt().decode(){
        Ok(args) => args,
        Err(docopt::Error::WithProgramUsage(_,usage)) |
        Err(docopt::Error::Usage(usage))              => {
            println!("{}",usage);
            return;
        },
        e => e.unwrap()
    };

    if args.flag_version{
        println!(concat!("tetr v",env!("CARGO_PKG_VERSION")));
        return;
    }

    //Define the OpenGL version to be used
    let opengl = OpenGL::_3_2;

    //Create a window.
    let window = Window::new(
        WindowSettings::new(
            "Polyminos Falling",
            [args.flag_window_size.0,args.flag_window_size.1]
        )
        .exit_on_esc(true)
        .opengl(opengl)
    );

    let (input_sender,input_receiver) = sync::mpsc::channel();

    fn imprint_cell(variant: &RotatedShape) -> cell::ShapeCell{
        cell::ShapeCell(Some(variant.shape()))
    }

    //Create a new application
    let mut app = App{
        gl: GlGraphics::new(opengl),
        tetris: GameState::new(
            rand::StdRng::new().unwrap(),
            imprint_cell as fn(&RotatedShape) -> cell::ShapeCell,
        ),
        paused: false,
        controllers: Vec::new(),
        input_receiver: input_receiver,
        connection: match args.flag_online{
            //No connection
            command_arg::OnlineConnection::none => online::ConnectionType::None,

            //Start to act as a client, connecting to a server
            command_arg::OnlineConnection::client => match net::UdpSocket::bind((net::Ipv4Addr::new(0,0,0,0),7375)){
                Ok(socket) => {
                    println!("Client: Connecting to {}:{}...",args.flag_host.0,args.flag_port);

                    //Send connect packet
                    socket.send_to(
                        online::client::packet::Connect{
                            protocol_version: u16_le::from(1)
                        }.into_packet().as_bytes(),
                        (args.flag_host.0,args.flag_port)
                    ).unwrap();

                    online::ConnectionType::Client(socket,net::SocketAddr::new(args.flag_host.0,args.flag_port))
                },
                Err(e) => {
                    println!("Client socket error: {:?}",e);
                    online::ConnectionType::None
                }
            },

            //Start to act as a server, listening for clients
            command_arg::OnlineConnection::server => match net::UdpSocket::bind((args.flag_host.0,args.flag_port)){
                Ok(socket) => {
                    use core::mem;

                    println!("Server: Listening on {}:{}...",args.flag_host.0,args.flag_port);
                    let input_sender = input_sender.clone();
                    thread::spawn(move ||{
                        use online::client::packet::*;

                        let mut buffer: PacketBytes = [0; online::client::packet::SIZE];
                        while let Ok((buffer_size,address)) = socket.recv_from(&mut buffer){
                            //First byte is the packet type
                            match Type::from_packet_bytes(&buffer[..]){
                                //Recevied connection request
                                Some(Type::Connect) if buffer_size==mem::size_of::<online::Packet<Type,Connect>>() => {
                                    let packet = Connect::from_packet_bytes(&buffer[..buffer_size]);
                                    print!("Server: Connection request from {}... ",address);
                                    match packet.protocol_version.into(){
                                        1 => {
                                            println!("OK");
                                            socket.send_to(
                                                online::server::packet::ConnectionEstablished{
                                                    connection_id: u32_le::from(7357)
                                                }.into_packet().as_bytes(),
                                                address
                                            ).unwrap();
                                        },

                                        version => {
                                            println!("Invalid version: {}",version);
                                            socket.send_to(online::server::packet::ConnectionInvalid.into_packet().as_bytes(),address).unwrap();
                                        }
                                    }
                                },

                                //Recevied player input
                                Some(Type::PlayerInput) if buffer_size==mem::size_of::<online::Packet<Type,PlayerInput>>() => {
                                    let packet = PlayerInput::from_packet_bytes(&buffer[..buffer_size]);
                                    match Input::from_u8(packet.input){
                                        Some(input) => input_sender.send((input,1)).unwrap(),
                                        None => ()
                                    }
                                },

                                //Received unimplemented TODO stuff
                                Some(ty) => println!("{:?}: {:?} (Size: {})",ty,buffer,buffer_size),

                                //Received other stuff
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

    {let App{tetris: ref mut game,controllers: ref mut cs,..} = app;
        //Create player 0
        game.add_player(0,player::Settings{
            gravityfall_frequency: 1.0,
            slowfall_delay       : 1.0,
            slowfall_frequency   : 1.0,
            move_delay           : 1.0,
            move_frequency       : 1.0,
            fastfall_shadow      : true,
        },&mut |e| for c in cs.iter_mut(){c.event(e);});

        //Create player 1
        game.add_player(1,player::Settings{
            gravityfall_frequency: 1.0,
            slowfall_delay       : 1.0,
            slowfall_frequency   : 1.0,
            move_delay           : 1.0,
            move_frequency       : 1.0,
            fastfall_shadow      : true,
        },&mut |e| for c in cs.iter_mut(){c.event(e);});

        //Create player 2
        /*cs.push(Box::new(ai::bruteforce::Controller::new(//TODO: Controllers shoulld probably be bound to the individual players
            input_sender.clone(),
            2,
            ai::bruteforce::Settings::default()
        )));
        game.add_player(1,player::Settings{
            gravityfall_frequency: 1.0,
            slowfall_delay       : 1.0,
            slowfall_frequency   : 1.0,
            move_delay           : 1.0,
            move_frequency       : 1.0,
            fastfall_shadow: false,
        },&mut |e| for c in cs.iter_mut(){c.event(e);});*/
    }

    //Run the created application: Listen for events
    for e in window.events(){
        //Player inflicted input: Keyboard events
        if let Some(Button::Keyboard(k)) = e.press_args(){
            app.on_key_press(k,&input_sender);
        }

        //Update
        if let Some(u) = e.update_args(){
            app.update(&u);
        }

        //Render
        if let Some(r) = e.render_args(){
            if app.paused{
                render::default::pause(&mut app.tetris,&mut app.gl,&r);
            }else{
                render::default::gamestate(&mut app.tetris,&mut app.gl,&r);
            }
        }
    }
}
