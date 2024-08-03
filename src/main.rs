use framebuffer::Framebuffer;
use loader::load_maze;
use minifb::{Key, Window, WindowOptions};
use player::Player;
use ray_caster::cast_ray;
use std::{time::Duration};

mod start_screen;
mod ray_caster;
mod player;
mod color;
mod framebuffer;
mod loader;
mod process_events;


fn draw_block(framebuffer: &mut framebuffer::Framebuffer, xo: usize, yo: usize, block_size: usize){
    let x1 = xo*block_size;
    let y1 = yo*block_size;
    for i in 0..block_size{
        for j in 0..block_size{
            framebuffer.point(x1+i, y1+j)
        }
    }
}
fn rener2d(){
 
}

fn render(framebuffer: &mut framebuffer::Framebuffer, val: char, x: usize, y: usize, block_size: usize) {
    match val {
        'g' => {
            framebuffer.set_current_color(0x03fc0f);
            draw_block(framebuffer, x, y, block_size)
        },
        '-' => {
            draw_block(framebuffer, x, y, block_size)
        },
        '|' => {
            draw_block(framebuffer, x, y, block_size)
        },
        '+' => {
            draw_block(framebuffer, x, y, block_size)
        },
        _ => (),
    }
    framebuffer.set_current_color(0xffffff);
}

fn render3d(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    block_size: usize,
){
    framebuffer.clear();
    let num_rays = framebuffer.width;

    for i in 0..num_rays{
        let current_ray = i as f32/ num_rays as f32;
        let a = player.a -(player.fov/2.0)+(player.fov*current_ray);
        let intersect = cast_ray(framebuffer, maze, player, a, block_size, false);

        let d_to_wall = if intersect.distance>10.0 {intersect.distance} else {10.0};
        let d_to_plane: f32 = 10.0;

        let hh =(framebuffer.height/2) as f32;
        let stake_height = (hh as f32/d_to_wall)*d_to_plane;
        let stake_top = (hh +(stake_height/2.0)) as usize;
        let stake_bottom = (hh -(stake_height/2.0)) as usize;
        for y in stake_bottom..stake_top{
                framebuffer.point(i, y);
        }
    }
}
fn draw_player_view(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    block_size: usize,
){
    let num_rays = 5;
    for i in 0..num_rays{
        let current_ray = i as f32/ num_rays as f32;
        let a = player.a -(player.fov/2.0)+(player.fov*current_ray);
        cast_ray(framebuffer, maze, player, a, block_size, true);
    }
}

fn pre_play(framebuffer: &mut Framebuffer,){
    start_screen::renderScreen(framebuffer);
}

fn main() {
    let maze = load_maze("./maze_easy.txt");


    let window_width = 600;
    let window_height = 600;
    let block_size = 50;

    let mut player = Player::new();
    let framebuffer_width = maze.len()*block_size;
    let framebuffer_height = maze[0].len()*block_size;

    let frame_delay = Duration::from_millis(70);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    
    // for row in 0..maze.len(){
    //     for col in 0..maze[row].len(){
    //         if maze[row][col] =='p'{
    //             player.setPos((row*block_size) as f32, (col*block_size) as f32);
    //         }
    //         render(&mut framebuffer, maze[row][col], row, col, block_size);
    //     }
    // }

    let mut window = Window::new(
        "rust grahpics - test",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut wall = false;
    let mut mode = "2D";
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        pre_play(&mut framebuffer);
        // if window.is_key_down(Key::C) {
        //     draw_player_view(&mut framebuffer, &maze, &mut player, block_size);
        // }
        // if window.is_key_down(Key::M){
        //     mode = if mode == "2D" {"3D"} else {"2D"}
        // }
        // if mode == "2D"{
        //     draw_player_view(&mut framebuffer, &maze, &mut player, block_size);
        // } else {
        //     render3d(&mut framebuffer, &maze, &mut player, block_size)
        // }

        // let intersect = cast_ray(&mut framebuffer, &maze, &player, player.a, block_size, false);
        // if intersect.distance < 5.0{
        //     wall = true;
        // } else{
        //     wall = false;
        // }
        // process_events::processEvent(&mut player, &window, wall);

        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
