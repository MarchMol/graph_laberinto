use crate::sprite_loader;
use crate::framebuffer::Framebuffer;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::Duration;

pub fn pre_play(val: &mut f32, screen: &mut usize) {
    // Starting screen window/framebuffer declarations
    let easy = sprite_loader::Sprite::new("./src/sprites/screens/start/start_easy.bmp");
    let mid = sprite_loader::Sprite::new("./src/sprites/screens/start/start_mid.bmp");
    let hard = sprite_loader::Sprite::new("./src/sprites/screens/start/start_hard.bmp");

    // Initializing Variables
    let window_width = 600;
    let window_height = 600;
    let framebuffer_width = 600;
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
        // Cases for closing window
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::Enter) {
            *screen = 1;
            break;
        }
        // Selection event
        select_listener(&window, val);
        //Rendering image
        if *val == 1.0 {
            sprite_loader::Sprite::render_screen(&mut framebuffer, &hard);
        } else if *val == 2.0 {
            sprite_loader::Sprite::render_screen(&mut framebuffer, &mid);
        } else {
            sprite_loader::Sprite::render_screen(&mut framebuffer, &easy);
        }
        //Updating Window
        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(0));
    }
}

pub fn select_listener(window: &Window, val: &mut f32) {
    if window.is_key_pressed(Key::Up, KeyRepeat::No) {
        *val += 1.0;
    }
    if window.is_key_pressed(Key::Down, KeyRepeat::No) {
        *val -= 1.0;
    }
    *val = val.rem_euclid(3.0);
}

pub fn post_play(val: &mut f32) {
    // Starting screen window/framebuffer declarations
    let mut animation_1 = sprite_loader::Sprite::new("./src/sprites/screens/end/win_easy_1.bmp");
    let mut animation_2 = sprite_loader::Sprite::new("./src/sprites/screens/end/win_easy_2.bmp");
    if *val==1.0{ // Hard
        animation_1 = sprite_loader::Sprite::new("./src/sprites/screens/end/win_hard_1.bmp");
        animation_2 = sprite_loader::Sprite::new("./src/sprites/screens/end/win_hard_2.bmp");
    } else if *val==2.0{
        animation_1 = sprite_loader::Sprite::new("./src/sprites/screens/end/win_mid_1.bmp");
        animation_2 = sprite_loader::Sprite::new("./src/sprites/screens/end/win_mid_2.bmp");
    }
    // Initializing Variables
    let mut animation_boolean = false;
    let window_width = 600;
    let window_height = 600;
    let framebuffer_width = 600;
    let framebuffer_height = 600;
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    sprite_loader::Sprite::render_screen(&mut framebuffer, &animation_1);
    let mut window = Window::new(
        "rust grahpics - test",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    // Starting menu window loop
    while window.is_open() {
        // Cases for closing window
        if window.is_key_down(Key::Escape) | window.is_key_down(Key::Enter){
            break;
        }
        //Rendering image
        if animation_boolean {
            sprite_loader::Sprite::render_screen(&mut framebuffer, &animation_1);
        } else{
            sprite_loader::Sprite::render_screen(&mut framebuffer, &animation_2);
        }
        animation_boolean=!animation_boolean;
        //Updating Window
        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(250));
    }
}

