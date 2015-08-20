#![feature(associated_consts,collections,core,ip_addr,lookup_host,optin_builtin_traits,plugin,raw,slice_patterns,str_split_at)]

#![plugin(docopt_macros)]
extern crate collections;
extern crate core;
extern crate docopt;
extern crate endian_type;
#[macro_use] extern crate enum_primitive;
extern crate fixed_circular_buffer;
extern crate graphics;
extern crate num;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate rustc_serialize;
extern crate vec_map;
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
use core::f64;
use endian_type::types::*;
use piston::window::WindowSettings;
use piston::event_loop::Events;
use piston::input::{Button,Key,PressEvent,ReleaseEvent,RenderEvent,UpdateEvent,UpdateArgs};
use opengl_graphics::{GlGraphics,OpenGL};
use std::{net,sync};
use std::collections::hash_map::{self,HashMap};
#[cfg(feature = "include_sdl2")]  use sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")]  use glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")]use glutin_window::GlutinWindow as Window;

use controller::ai;
use data::{cell,player};
use data::grid::{self,Grid};
use data::map::dynamic_map::Map;
use data::player::Player;
use data::shapes::tetromino::{Shape,RotatedShape};
use data::input::Input;
use gamestate::{GameState,MapId,PlayerId};
use game::event::Event;
use tmp_ptr::TmpPtr;

struct App{
    gl: GlGraphics,
    game_state: GameState<Map<cell::ShapeCell>,rand::StdRng>,
    controllers: Vec<Box<Controller<Map<cell::ShapeCell>,Event<(PlayerId,TmpPtr<Player>),(MapId,TmpPtr<Map<cell::ShapeCell>>)>>>>,
    input_receiver: sync::mpsc::Receiver<(Input,PlayerId)>,
    connection: online::ConnectionType,
    paused: bool,
    key_map: data::input::key::KeyMap,
    key_down: HashMap<Key,f64>,
}

impl App{
    fn update(&mut self, args: &UpdateArgs,input_sender: &sync::mpsc::Sender<(Input,PlayerId)>){
        //Controllers
        if !self.paused{
            for mut controller in self.controllers.iter_mut(){
                controller.update(args,&self.game_state.players,&self.game_state.maps);
            }
        }

        //Key repeat
        for (key,time_left) in self.key_down.iter_mut(){
            while{
                *time_left-= args.dt;
                *time_left <= 0.0
            }{
                *time_left = if let Some(mapping) = self.key_map.get(&key){
                    input_sender.send((mapping.input,mapping.player)).unwrap();
                    *time_left + mapping.repeat_frequency
                }else{
                    f64::NAN//TODO: If the mapping doesn't exist, it will never be removed
                }
            }
        }

        //Input
        while let Ok((input,pid)) = self.input_receiver.try_recv(){
            if let Some(player) = self.game_state.players.get_mut(&(pid as usize)){
                if let Some(map) = self.game_state.maps.get_mut(&(player.map as usize)){
                    input::perform(input,player,map);
                }
            }

            if let online::ConnectionType::Client(ref socket,ref address) = self.connection{if pid==0{
                socket.send_to(online::client::packet::PlayerInput{
                    connection_id: u32_le::from(0),
                    player_network_id: u32_le::from(0),
                    input: input as u8
                }.into_packet(u16_le::from(0)).as_bytes(),address).unwrap();
            }}
        }

        //Update
        if !self.paused{
            let &mut App{game_state: ref mut game,controllers: ref mut cs,..} = self;
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
            Key::D1     => {if let Some(player) = self.game_state.players.get_mut(&(0 as usize)){player.shape = RotatedShape::new(Shape::I);};},
            Key::D2     => {if let Some(player) = self.game_state.players.get_mut(&(0 as usize)){player.shape = RotatedShape::new(Shape::L);};},
            Key::D3     => {if let Some(player) = self.game_state.players.get_mut(&(0 as usize)){player.shape = RotatedShape::new(Shape::O);};},
            Key::D4     => {if let Some(player) = self.game_state.players.get_mut(&(0 as usize)){player.shape = RotatedShape::new(Shape::J);};},
            Key::D5     => {if let Some(player) = self.game_state.players.get_mut(&(0 as usize)){player.shape = RotatedShape::new(Shape::T);};},
            Key::D6     => {if let Some(player) = self.game_state.players.get_mut(&(0 as usize)){player.shape = RotatedShape::new(Shape::S);};},
            Key::D7     => {if let Some(player) = self.game_state.players.get_mut(&(0 as usize)){player.shape = RotatedShape::new(Shape::Z);};},
            Key::R      => {
                match self.game_state.players.get(&(0 as usize)).map(|player| player.map){//TODO: New seed for rng
                    Some(map_id) => {
                        let &mut App{game_state: ref mut game,controllers: ref mut cs,..} = self;
                        game.reset_map(map_id,&mut |e| for c in cs.iter_mut(){c.event(e);});
                    },
                    None => ()
                };
            },
            Key::Home   => {if let Some(player) = self.game_state.players.get_mut(&(0 as usize)){player.pos.y = 0;};},

            //Other keys, check key bindings
            key => if let Some(mapping) = self.key_map.get(&key){
                if let hash_map::Entry::Vacant(entry) = self.key_down.entry(key){
                    entry.insert(mapping.repeat_delay);
                    input_sender.send((mapping.input,mapping.player)).unwrap();
                }
            }
        }}
    }

    fn on_key_release(&mut self,key: Key){
        if let hash_map::Entry::Occupied(entry) = self.key_down.entry(key){
            entry.remove();
        }
    }
}

//Constants
macro_rules! PROGRAM_NAME{() => ("tetr")}
macro_rules! PROGRAM_NAME_VERSION{() => (concat!(PROGRAM_NAME!()," v",env!("CARGO_PKG_VERSION")))}

docopt!(Args derive Debug,concat!("
Usage: ",PROGRAM_NAME!()," [options]
       ",PROGRAM_NAME!()," --help

A game with tetrominos falling.

Options:
  -h, --help           Show this message
  -v, --version        Show version
  --credits            Show credits/staff
  --manual             Show instruction manual/guide for the game
  --online=CONNECTION  Available modes: none, server, client [default: none]
  --host=ADDR          Network address used for the online connection [default: 0.0.0.0]
  --port=N             Network port used for the online connection [default: 7374]
  --window-size=SIZE   Window size [default: 800x600]
  --window-mode=MODE   Available modes: window, fullscreen [default: window]
"),
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
        println!(PROGRAM_NAME_VERSION!());
        return;
    }

    if args.flag_credits{
        println!(concat!(PROGRAM_NAME_VERSION!(),": Credits\n\n",include_str!("../CREDITS")));
        return;
    }

    if args.flag_manual{
        println!(concat!(PROGRAM_NAME_VERSION!(),": Instruction manual\n\n",include_str!("../MANUAL")));
        return;
    }

    //Define the OpenGL version to be used
    let opengl = OpenGL::V3_2;

    //Create a window.
    let window = Window::new(
        WindowSettings::new(
            concat!(PROGRAM_NAME!()," v",env!("CARGO_PKG_VERSION")),
            [args.flag_window_size.0,args.flag_window_size.1]
        )
        .exit_on_esc(true)
        .opengl(opengl)
    ).unwrap();

    let (input_sender,input_receiver) = sync::mpsc::channel();

    //Create a new application
    let mut app = App{
        gl: GlGraphics::new(opengl),
        game_state: GameState::new(
            rand::StdRng::new().unwrap(),
            {fn f(variant: &RotatedShape) -> cell::ShapeCell{
                cell::ShapeCell(Some(variant.shape()))
            }f}as fn(&_) -> _,
            {fn f<M: Grid>(shape: &RotatedShape,map: &M) -> grid::Pos{grid::Pos{
                x: map.width() as grid::PosAxis/2 - shape.center_x() as grid::PosAxis,
                y: 0//TODO: Optionally spawn above: `-(shape.height() as grid::PosAxis);`. Problem is the collision checking. And this is not how it usually is done in other games
            }}f::<Map<cell::ShapeCell>>}as fn(&_,&_) -> _,
        ),
        paused: false,
        controllers: Vec::new(),
        key_map: HashMap::new(),
        key_down: HashMap::new(),
        input_receiver: input_receiver,
        connection: match args.flag_online{
            //No connection
            command_arg::OnlineConnection::none => online::ConnectionType::None,

            //Start to act as a client, connecting to a server
            command_arg::OnlineConnection::client => {
                let server_addr = net::SocketAddr::new(args.flag_host.0,args.flag_port);

                match online::client::start(server_addr,input_sender.clone()){
                    Ok(socket) => online::ConnectionType::Client(socket,server_addr),
                    Err(_)     => online::ConnectionType::None
                }
            },

            //Start to act as a server, listening for clients
            command_arg::OnlineConnection::server => {
                let server_addr = net::SocketAddr::new(args.flag_host.0,args.flag_port);

                match online::server::start(server_addr,input_sender.clone()){
                    Ok(_)  => online::ConnectionType::Server,
                    Err(_) => online::ConnectionType::None
                }
            }
        },
    };

    //Create map
    app.game_state.maps.insert(0,Map::new(10,20));
    app.game_state.maps.insert(1,Map::new(10,20));

    {let App{game_state: ref mut game,controllers: ref mut cs,..} = app;
        if let online::ConnectionType::None = app.connection{
            //Create player 0
            game.rngs.insert_from_global(gamestate::data_map::MappingKey::Player(0));
            game.add_player(0,player::Settings{
                gravityfall_frequency: 1.0,
                fastfall_shadow      : true,
            },&mut |e| for c in cs.iter_mut(){c.event(e);});

            //Create player 1
            game.rngs.insert_from_global(gamestate::data_map::MappingKey::Player(1));
            cs.push(Box::new(ai::bruteforce::Controller::new(//TODO: Controllers shoulld probably be bound to the individual players
                input_sender.clone(),
                1,
                ai::bruteforce::Settings::default()
            )));
            game.add_player(1,player::Settings{
                gravityfall_frequency: 1.0,
                fastfall_shadow      : true,
            },&mut |e| for c in cs.iter_mut(){c.event(e);});
        }
    }

    {//Key mappings
        use data::input::key::Mapping;

        //Player 0
        app.key_map.insert(Key::Left   ,Mapping{input: Input::MoveLeft,           player: 0,repeat_delay: 0.3,repeat_frequency: 0.1});
        app.key_map.insert(Key::Right  ,Mapping{input: Input::MoveRight,          player: 0,repeat_delay: 0.3,repeat_frequency: 0.1});
        app.key_map.insert(Key::Down   ,Mapping{input: Input::SlowFall,           player: 0,repeat_delay: 0.3,repeat_frequency: 0.07});
        app.key_map.insert(Key::End    ,Mapping{input: Input::FastFall,           player: 0,repeat_delay: f64::NAN,repeat_frequency: f64::NAN});
        app.key_map.insert(Key::X      ,Mapping{input: Input::RotateAntiClockwise,player: 0,repeat_delay: 0.3,repeat_frequency: 0.2});
        app.key_map.insert(Key::Z      ,Mapping{input: Input::RotateClockwise,    player: 0,repeat_delay: 0.3,repeat_frequency: 0.2});

        //Player 1
        app.key_map.insert(Key::NumPad4,Mapping{input: Input::MoveLeft,           player: 1,repeat_delay: 0.3,repeat_frequency: 0.1});
        app.key_map.insert(Key::NumPad6,Mapping{input: Input::MoveRight,          player: 1,repeat_delay: 0.3,repeat_frequency: 0.1});
        app.key_map.insert(Key::NumPad5,Mapping{input: Input::SlowFall,           player: 1,repeat_delay: 0.3,repeat_frequency: 0.07});
        app.key_map.insert(Key::NumPad2,Mapping{input: Input::FastFall,           player: 1,repeat_delay: f64::NAN,repeat_frequency: f64::NAN});
        app.key_map.insert(Key::NumPad1,Mapping{input: Input::RotateAntiClockwise,player: 1,repeat_delay: 0.3,repeat_frequency: 0.2});
        app.key_map.insert(Key::NumPad0,Mapping{input: Input::RotateClockwise,    player: 1,repeat_delay: 0.3,repeat_frequency: 0.2});
    }

    //Run the created application: Listen for events
    for e in window.events(){
        //Player inflicted input: Keyboard events
        if let Some(Button::Keyboard(k)) = e.press_args(){
            app.on_key_press(k,&input_sender);
        }
        if let Some(Button::Keyboard(k)) = e.release_args(){
            app.on_key_release(k);
        }

        //Update
        if let Some(u) = e.update_args(){
            app.update(&u,&input_sender);//TODO: It "feels" not good having to give the input sender to the update method, where it should be receive input
        }

        //Render
        if let Some(r) = e.render_args(){
            if app.paused{
                render::default::pause(&mut app.game_state,&mut app.gl,&r);
            }else{
                render::default::gamestate(&mut app.game_state,&mut app.gl,&r);
            }
        }
    }
}
