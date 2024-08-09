use framebuffer::Framebuffer;
use loader::load_maze;
use minifb::{Key, Window, WindowOptions};
use player::Player;
use ray_caster::{cast_ray, Intersect};
use std::time::{ Instant,Duration};
use core::f32::consts::PI;

mod start_screen;
mod ray_caster;
mod player;
mod color;
mod framebuffer;
mod loader;
mod process_events;
mod fps;


fn draw_block(framebuffer: &mut framebuffer::Framebuffer, xo: usize, yo: usize, block_size: usize){
    for i in 0..block_size{
        for j in 0..block_size{
            framebuffer.point(xo+i, yo+j)
        }
    }
}

fn render2d(
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
                    player.setPos((row*block_size+5) as f32, (col*block_size+5) as f32);
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
        let d_to_plane: f32 = 15.0;

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
    framebuffer.clear();
    draw_block(framebuffer, player.pos.x as usize ,player.pos.y as usize, 5);
    framebuffer.set_current_color(0xffffff);
    render2d(framebuffer, maze, block_size, player, false);
    let num_rays = 5;
    
    for i in 0..num_rays{
        let current_ray = i as f32/ num_rays as f32;
        let a = player.a -(player.fov/2.0)+(player.fov*current_ray);
        cast_ray(framebuffer, maze, player, a, block_size, true);
    }
}


fn playing(val: f32, screen: &mut usize){
    // Loading correct maze
    let mut maze_name = "./maze_easy.txt";
    match val{
        1.0 =>{
            maze_name = "./maze_medium.txt";
        },
        2.0=>{
            maze_name = "./maze_hard.txt";
        },
        _ => (),
    };
    let maze = load_maze(maze_name);

    // loading numbers for fps
    let numbers = load_maze("./numbers.txt");

    // intialising variables
    let window_width = 600;
    let window_height = 600;

    let block_size = 40;

    let framebuffer_width = maze.len()*block_size;
    let framebuffer_height = maze[0].len()*block_size;

    let mut player = Player::new();

    let frame_delay = Duration::from_millis(0);
    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    // drawing 2d maze
    render2d(&mut framebuffer, &maze, block_size, &mut player, true);

    let mut window = Window::new(
        "rust grahpics - test",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut wall_f = false;
    let mut wall_b = false;
    let mut mode = "2D";
    let mut last_time = Instant::now();
    let mut fps_counter = 0;
    let mut fps_last = 10;
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No){
            mode = if mode == "2D" {"3D"} else {"2D"}
        }

        // Game Mode
        if mode == "2D"{ // 2D
            draw_player_view(&mut framebuffer, &maze, &mut player, block_size);
        } else { // 3D
            render3d(&mut framebuffer, &maze, &mut player, block_size);
        }

        // Fps Counter
        fps_counter += 1;
        if last_time.elapsed() >= Duration::from_secs(1) {
            fps_last = fps_counter;
            fps_counter = 0;
            last_time = Instant::now();
        }
        fps::render_fps(&mut framebuffer, &numbers, fps_last);

        // Intersection controll (front or back wall are too close)
        let intersect_f = cast_ray(&mut framebuffer, &maze, &player, player.a, block_size, false);
        let intersect_b = cast_ray(&mut framebuffer, &maze, &player, player.a+PI, block_size, false);
        if intersect_f.distance < 6.0{
            wall_f = true;
        } else{
            wall_f = false;
        }
        if intersect_b.distance < 6.0{
            wall_b = true;
        } else{
            wall_b = false;
        }
        process_events::processEvent(&mut player, &window, wall_f, wall_b);

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
fn main() {
    let mut val: f32 = 0.0;
    let mut screen: usize = 0;

    start_screen::pre_play(&mut val, &mut screen);
    if screen!=0{
        playing(val, &mut screen);
    }

}
