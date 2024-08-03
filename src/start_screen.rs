use minifb::{Key, Window};

use crate::framebuffer::Framebuffer;
use crate::loader::load_maze;

pub fn renderScreen(framebuffer: &mut Framebuffer){
    let mut block_size = 10;
    let spacing = 10;
    let start = load_maze("./start_screen.txt");

    for row in 0..start.len(){
        for col in 0..start[row].len(){
            if row>6{
                block_size = 5;
            }
            if start[row][col]!=' '{
                renderBlock(framebuffer, row*block_size, col*block_size, block_size)
            }
        }
    }
}
pub fn renderBlock(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize){
    for i in xo..xo+block_size{
        for j in yo..yo+block_size{
            framebuffer.point(j, i)
        }
    }
}

pub fn renderSelect(framebuffer: &mut Framebuffer, val: usize){

}

pub fn selectListener(framebuffer: &mut Framebuffer, window: &Window){
    if window.is_key_down(Key::Up){

    }
}