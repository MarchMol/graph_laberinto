use crate::framebuffer;
use crate::player::Player;
use crate::Goal;

pub fn init_maze(
    framebuffer: &mut framebuffer::Framebuffer, 
    maze: &Vec<Vec<char>>,
    block_size: usize,
    player: &mut Player,
    goal: &mut Goal,
){

    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
                if maze[row][col] =='p'{
                    player.set_pos((row*block_size) as f32,
                     (col*block_size) as f32);
                } else if maze[row][col] =='g'{
                    goal.pos.x =((row)*block_size +block_size/2) as f32;
                    goal.pos.y = (col*block_size +block_size/2) as f32;
                }
            
                match maze[row][col] {
                    'g' =>(),
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

pub fn render2d(
    framebuffer: &mut framebuffer::Framebuffer, 
    maze: &Vec<Vec<char>>,
    block_size: usize,
    player: &mut Player,
    minimaze: bool
){
    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
                match maze[row][col] {
                    'g' => {
                        framebuffer.set_current_color(0x03fc0f);
                        draw_block(framebuffer,
                             ((row as f32+0.25)*block_size as f32) as usize, 
                             ((col as f32+0.25)*block_size as f32) as usize, 
                             block_size/2);
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
    if !minimaze{
        if (maze[(player.pos.x/block_size as f32) as usize][(player.pos.y/block_size as f32) as usize])=='g'{
            player.win_condition=true;
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
