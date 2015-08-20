pub mod default{
    use graphics::{self,Transformed};
    use opengl_graphics::GlGraphics;
    use piston::input::RenderArgs;

    use data::{cell,colors,grid};
    use data::map::Map;
    use data::shapes::tetromino::Shape;
    use gamestate::GameState;

    ///Renders the pause state
    pub fn pause<M,Rng>(state: &mut GameState<M,Rng>,gl: &mut GlGraphics,args: &RenderArgs)
        where M: Map<Cell = cell::ShapeCell>
    {
        gamestate(state,gl,args);

        //Pause overlay
        gl.draw(args.viewport(),|context,gl|{
            let [w,h] = context.get_view_size();
            graphics::rectangle([0.0,0.0,0.0,0.5],[0.0,0.0,w,h],context.transform,gl);
        });
    }

    ///Renders the game state
    pub fn gamestate<M,Rng>(state: &mut GameState<M,Rng>,gl: &mut GlGraphics,args: &RenderArgs)
        where M: Map<Cell = cell::ShapeCell>
    {
        const BLOCK_PIXEL_SIZE: f64 = 24.0;

        fn map_render_pos(map_no: usize) -> (f64,f64){
            (map_no as f64 * 12.0 * BLOCK_PIXEL_SIZE,0.0)
        }

        //Unit square
        let square = graphics::rectangle::square(0.0,0.0,BLOCK_PIXEL_SIZE);

        //Draw in the current viewport
        gl.draw(args.viewport(),|context,gl|{
            //Clear screen
            graphics::clear(colors::BLACK,gl);

            //Draw maps
            for (map_id,map) in state.maps.iter(){
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
            for (_,player) in state.players.iter(){match state.maps.get(&(player.map as usize)){
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
        });
    }
}
