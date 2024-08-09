use crate::framebuffer;
use crate::player::Player;

pub fn render2d(
    framebuffer: &mut framebuffer::Framebuffer, 
    maze: &Vec<Vec<char>>,
    block_size: usize,
    player: &mut Player,
    init: bool,
){
    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
            if init{
                if maze[row][col] =='p'{
                    player.setPos((row*block_size) as f32, (col*block_size) as f32);
                }
            } else{
                match maze[row][col] {
                    'g' => {
                        framebuffer.set_current_color(0x03fc0f);
                        draw_block(framebuffer, row*block_size, col*block_size, block_size);
                    },
                    ' ' => (),
                    'p' =>(),
                    _ => {
                        draw_block(framebuffer, row*block_size, col*block_size, block_size);
                    },
                }
                framebuffer.set_current_color(0xffffff);
            }
            
        }
    }
}

pub fn draw_block(framebuffer: &mut framebuffer::Framebuffer, xo: usize, yo: usize, block_size: usize){
    for i in 0..block_size{
        for j in 0..block_size{
            framebuffer.point(xo+i, yo+j)
        }
    }
}
