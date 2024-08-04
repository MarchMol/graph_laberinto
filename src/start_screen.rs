use minifb::{Key, KeyRepeat, Window};

use crate::framebuffer::Framebuffer;
use crate::loader::load_maze;

pub fn renderScreen(framebuffer: &mut Framebuffer){
    let mut block_size = 10;
    let mut spacing = 0;
    let start = load_maze("./start_screen.txt");
    framebuffer.set_current_color(0x000000);
    for row in 0..start.len(){
        for col in 0..start[row].len(){
            if row>46{
                block_size = 3;
                spacing = 150;
            } else if row>10 {
                block_size = 7;
            }
            if start[row][col]!=' '{
                renderBlock(framebuffer, row*block_size+spacing, col*block_size, block_size)
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

pub fn renderRectangle(framebuffer: &mut Framebuffer, xo: usize, yo: usize, w: usize, h: usize){
    for i in xo..xo+h{
        for j in yo..yo+w{
            framebuffer.point(j, i);
        }
    }
}

pub fn renderSelect(framebuffer: &mut Framebuffer, val: &mut f32){
    framebuffer.set_bgcolor(0x8c65a1);
    framebuffer.clear();
    if *val==0.0{
        framebuffer.set_current_color(0x09ff00);
        renderRectangle(framebuffer, 150, 100, 200,50);
    } else if *val==1.0{
        framebuffer.set_current_color(0xffd900);
        renderRectangle(framebuffer, 220, 100, 200,50);
    } else{
        framebuffer.set_current_color(0xff0000);
        renderRectangle(framebuffer, 275, 100, 200,50);
    }
    framebuffer.set_current_color(0x000000);
    
}

pub fn selectListener(framebuffer: &mut Framebuffer, window: &Window, val: &mut f32){
    if window.is_key_pressed(Key::Up, KeyRepeat::No){
        *val-=1.0;
    }
    if window.is_key_pressed(Key::Down, KeyRepeat::No){
        *val+=1.0;
    }
    *val = val.rem_euclid(3.0);
}