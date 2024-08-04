use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::Duration;

use crate::framebuffer::Framebuffer;
use crate::loader::load_maze;

pub fn pre_play(val: &mut f32, screen: &mut usize){
    // Starting screen window/framebuffer declarations
    let window_width = 600;
    let window_height = 600;

    let framebuffer_width = 400;
    let framebuffer_height = 600;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "rust grahpics - test",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    // Starting menu window loop
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::Enter){
            *screen = 1;
            break;
        }
        select_listener(&window, val);
        render_select(&mut framebuffer, val);
        render_screen(&mut framebuffer);
        window
        .update_with_buffer(
            &framebuffer.color_array_to_u32(),
            framebuffer_width,
            framebuffer_height,
        )
        .unwrap();
    std::thread::sleep(Duration::from_millis(70));
    }
}


pub fn render_screen(framebuffer: &mut Framebuffer){
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
                render_rectangle(framebuffer, row*block_size+spacing, col*block_size, block_size, block_size)
            }
        }
    }
}

pub fn render_rectangle(framebuffer: &mut Framebuffer, xo: usize, yo: usize, w: usize, h: usize){
    for i in xo..xo+h{
        for j in yo..yo+w{
            framebuffer.point(j, i);
        }
    }
}

pub fn render_select(framebuffer: &mut Framebuffer, val: &mut f32){
    framebuffer.set_bgcolor(0x8c65a1);
    framebuffer.clear();
    if *val==0.0{
        framebuffer.set_current_color(0x09ff00);
        render_rectangle(framebuffer, 152, 100, 200,50);
    } else if *val==1.0{
        framebuffer.set_current_color(0xffd900);
        render_rectangle(framebuffer, 215, 100, 200,50);
    } else{
        framebuffer.set_current_color(0xff0000);
        render_rectangle(framebuffer, 272, 100, 200,50);
    }
    framebuffer.set_current_color(0x000000);
    
}

pub fn select_listener(window: &Window, val: &mut f32){
    if window.is_key_pressed(Key::Up, KeyRepeat::No){
        *val-=1.0;
    }
    if window.is_key_pressed(Key::Down, KeyRepeat::No){
        *val+=1.0;
    }
    *val = val.rem_euclid(3.0);
}