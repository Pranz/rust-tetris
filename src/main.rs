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
#[cfg(feature = "include_sdl2")]  use sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")]  use glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")]use glutin_window::GlutinWindow as Window;

use controller::ai;
use data::{cell,colors,player};
use data::grid::{self,Grid};
use data::map::dynamic_map::Map;
use data::shapes::tetrimino::{Shape,RotatedShape};
use gamestate::GameState;

struct App<Rng>{
    gl: GlGraphics,
    tetris: GameState<Map<cell::ShapeCell>,Rng>,
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
        let &mut App{ref mut gl,ref mut tetris} = self;
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
                            let transform = transform.trans((cell_pos.x as grid::PosAxis + player.pos.x) as f64 * BLOCK_PIXEL_SIZE, (cell_pos.y as grid::PosAxis + player.pos.y) as f64 * BLOCK_PIXEL_SIZE);
                            graphics::rectangle(color,square,transform,gl);
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
            Key::Left   => {self.tetris.with_player_map(0,|player,map|{gamestate::move_player(player,map,grid::Pos{x: -1,y: 0});});},
            Key::Right  => {self.tetris.with_player_map(0,|player,map|{gamestate::move_player(player,map,grid::Pos{x:  1,y: 0});});},
            Key::Down   => {self.tetris.with_player_map(0,|player,map|{
                player.move_time_count = if gamestate::move_player(player,map,grid::Pos{x: 0,y: 1}){
                    //Reset timer
                    0.0
                }else{
                    //Set timer and make the player move in the update step
                    player.settings.move_frequency
            };});},
            Key::Up     => {self.tetris.with_player_map(0,|player,map|{gamestate::rotate_anticlockwise_and_resolve_player(player,map);});},
            Key::X      => {self.tetris.with_player_map(0,|player,map|{gamestate::rotate_anticlockwise_and_resolve_player(player,map);});},
            Key::Z      => {self.tetris.with_player_map(0,|player,map|{gamestate::rotate_clockwise_and_resolve_player(player,map);});},

            //Player 1
            Key::NumPad4 => {self.tetris.with_player_map(1,|player,map|{gamestate::move_player(player,map,grid::Pos{x: -1,y: 0});});},
            Key::NumPad6 => {self.tetris.with_player_map(1,|player,map|{gamestate::move_player(player,map,grid::Pos{x:  1,y: 0});});},
            Key::NumPad5 => {self.tetris.with_player_map(1,|player,map|{
                player.move_time_count = if gamestate::move_player(player,map,grid::Pos{x: 0,y: 1}){
                    //Reset timer
                    0.0
                }else{
                    //Set timer and make the player move in the update step
                    player.settings.move_frequency
            };});},
            Key::NumPad1 => {self.tetris.with_player_map(1,|player,map|{gamestate::rotate_anticlockwise_and_resolve_player(player,map);});},
            Key::NumPad0 => {self.tetris.with_player_map(1,|player,map|{gamestate::rotate_clockwise_and_resolve_player(player,map);});},

            //Player 2
            Key::A => {self.tetris.with_player_map(2,|player,map|{gamestate::move_player(player,map,grid::Pos{x: -1,y: 0});});},
            Key::D => {self.tetris.with_player_map(2,|player,map|{gamestate::move_player(player,map,grid::Pos{x:  1,y: 0});});},
            Key::S => {self.tetris.with_player_map(2,|player,map|{
                player.move_time_count = if gamestate::move_player(player,map,grid::Pos{x: 0,y: 1}){
                    //Reset timer
                    0.0
                }else{
                    //Set timer and make the player move in the update step
                    player.settings.move_frequency
            };});},
            Key::LShift => {self.tetris.with_player_map(1,|player,map|{gamestate::rotate_anticlockwise_and_resolve_player(player,map);});},
            Key::Space  => {self.tetris.with_player_map(1,|player,map|{gamestate::rotate_clockwise_and_resolve_player(player,map);});},


            //Other keys
            _ => ()
        }}
    }
}

fn main(){
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

    //Create a new application
    let mut app = App{
        gl: GlGraphics::new(opengl),
        tetris: GameState::new(rand::StdRng::new().unwrap()),
    };

    //Create map
    app.tetris.maps.insert(0,Map::new(10,20));
    app.tetris.maps.insert(1,Map::new(10,20));

    //Create player 0
    app.tetris.add_player(0,player::Settings{
        move_frequency : 1.0,
    });

    //Create player 1
    /*let player1 = app.tetris.add_player(1,player::Settings{
        move_frequency : 1.0,
    }).unwrap();
    app.tetris.controllers.insert(player1 as usize,Box::new(ai::bounce::Controller::new()));
*/
    //Create player 2
    let player2 = app.tetris.add_player(1,player::Settings{
        move_frequency : 1.0,
    }).unwrap();
    app.tetris.controllers.insert(player2 as usize,Box::new(ai::fill_one::Controller::default()));

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
