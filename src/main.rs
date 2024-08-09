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

mod render2d;
mod render3d;

fn draw_player_view(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    block_size: usize,
    scale: usize,
){
    framebuffer.clear();
    framebuffer.set_current_color(0xff1100);
    render2d::draw_block(framebuffer, player.pos.x as usize ,player.pos.y as usize, block_size/6);
    framebuffer.set_current_color(0xffffff);
    render2d::render2d(framebuffer, maze, scale, player, false);
    let num_rays = 3;
    
    for i in 0..num_rays{
        let current_ray = i as f32/ num_rays as f32;
        let a = player.a -(player.fov/2.0)+(player.fov*current_ray);
        cast_ray(framebuffer, maze, player, a, block_size, true);
    }
}
fn draw_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    block_size: usize,
    scale: usize,
){
    framebuffer.set_current_color(0xff1100);
    render2d::draw_block(framebuffer, 
        (player.pos.x*scale as f32/block_size as f32) as usize ,
        (player.pos.y*scale as f32/block_size as f32) as usize ,
        block_size/6);
    framebuffer.set_current_color(0xffffff);
    render2d::render2d(framebuffer, maze, scale, player, false);
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
    render2d::render2d(&mut framebuffer, &maze, block_size, &mut player, true);

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
    let mut last_input = Instant::now();
    let mut fps_counter = 0;
    let mut fps_last = 10;
    
    while window.is_open(){
        // closing game
        if window.is_key_down(Key::Escape) {
            break;
        }
        // Toggle game mode
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No){
            mode = if mode == "2D" {"3D"} else {"2D"}
        }
        if mode == "2D"{ // 2D
            draw_player_view(&mut framebuffer, &maze, &mut player, block_size,block_size);
        } else { // 3D
            
            render3d::render3d(&mut framebuffer, &maze, &mut player, block_size);
            draw_minimap(&mut framebuffer, &maze, &mut player, block_size,5);
            
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
        if last_input.elapsed() >= Duration::from_millis(16) {
            let intersect_f = cast_ray(&mut framebuffer, &maze, &player, player.a, block_size, false);
            let intersect_b = cast_ray(&mut framebuffer, &maze, &player, player.a+PI, block_size, false);
            if intersect_f.distance < 6.0{
                if intersect_f.impact=='g'{
                    *screen = 3;
                    break;
                } else{
                    wall_f = true;
                }
            } else{
                wall_f = false;
            }
            if intersect_b.distance < 6.0{
                wall_b = true;
            } else{
                wall_b = false;
            }
            process_events::processEvent(&mut player, &window, wall_f, wall_b);
            last_input = Instant::now();
        }


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
    if screen==3{

    }

}
