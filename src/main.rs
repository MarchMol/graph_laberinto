use framebuffer::Framebuffer;
use loader::load_maze;
use minifb::{Key, Window, WindowOptions};
use player::Player;
use ray_caster::cast_ray;
use std::time::{ Instant,Duration};
use core::f32::consts::PI;
use nalgebra_glm::Vec2;

mod ray_caster;
mod player;
mod color;
mod framebuffer;
mod loader;
mod process_events;
mod fps;
mod sprite_loader;

mod screen_render;
mod render2d;
mod render3d;

struct Goal{
    pos: Vec2,
    sprite: sprite_loader::Sprite,
}
impl Goal{
    fn new(pos:Vec2, sprite:sprite_loader::Sprite) -> Self{
        Goal{
            pos,
            sprite,
        }
    }
}
fn draw_player_view(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    block_size: usize,
    scale: usize,
    goal: &Goal
){
    framebuffer.clear();
    framebuffer.set_current_color(0xff1100);
    render2d::draw_block(framebuffer, player.pos.x as usize-block_size/12 ,player.pos.y as usize-block_size/12, block_size/6);
    framebuffer.set_current_color(0xffffff);
    render2d::render2d(framebuffer, maze, scale, player,false);
    let num_rays = 3;
    
    for i in 0..num_rays{
        let current_ray = i as f32/ num_rays as f32;
        let a = player.a -(player.fov/2.0)+(player.fov*current_ray);
        cast_ray(framebuffer, maze, player, a, block_size, true, &goal);
    }
}
fn draw_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    block_size: usize,
    scale: usize,
){
    framebuffer.set_current_color(0x000000);
    for x in 0..maze.len()*scale{
        for y in 0..maze[0].len()*scale{
            framebuffer.point(x, y);
        }
    }
    framebuffer.set_current_color(0xff1100);
    render2d::draw_block(framebuffer, 
        (player.pos.x*scale as f32/block_size as f32) as usize -block_size/24 ,
        (player.pos.y*scale as f32/block_size as f32) as usize -block_size/24,
        block_size/12);
    framebuffer.set_current_color(0xffffff);
    render2d::render2d(framebuffer, maze, scale, player, true);
}


fn playing(val: f32, screen: &mut usize){
    // Calculating correct maze and prize according to difficulty
    let mut goal_name = "./src/sprites/prizes/prize_easy.bmp";
    let mut maze_name = "./src/mazes/maze_easy.txt";
    match val{
        1.0 =>{
            maze_name = "./src/mazes/maze_hard.txt";
            goal_name = "./src/sprites/prizes/prize_hard.bmp";
        },
        2.0=>{
            maze_name = "./src/mazes/maze_mid.txt";
            goal_name = "./src/sprites/prizes/prize_mid.bmp";
        },
        _ => (),
    };

    // Pre-Loading 
    let maze = load_maze(maze_name);
    let mut goal = Goal::new(
        Vec2::new(0.0, 0.0),
        sprite_loader::Sprite::new(goal_name));
    let numbers = load_maze("./src/mazes/numbers.txt");

    // intialising variables
    let window_width = 600;
    let window_height = 600;
    
    let block_size = 600/maze.len();

    let framebuffer_width = 600;
    let framebuffer_height = 600;

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut player = Player::new(block_size);
    let frame_delay = Duration::from_millis(0);

    // drawing 2d maze
    render2d::init_maze(&mut framebuffer, &maze, block_size, &mut player, &mut goal);

    let mut window = Window::new(
        "Rat-Lab",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut mode = "3D";
    let mut last_time = Instant::now();
    let mut last_input = Instant::now();
    let mut fps_counter = 0;
    let mut fps_last = 10;
    let wall_1 = sprite_loader::Sprite::new("./src/sprites/walls/wall_1.bmp");
    let wall_2 = sprite_loader::Sprite::new("./src/sprites/walls/wall_2.bmp");
    let wall_3 = sprite_loader::Sprite::new("./src/sprites/walls/wall_3.bmp");
    let sprites = [&wall_1, &wall_2, &wall_3];
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
            draw_player_view(&mut framebuffer, &maze, &mut player, block_size,block_size, &mut goal);
        } else { // 3D
            render3d::render3d(&mut framebuffer, &maze, &mut player, block_size, &sprites, &mut goal);
            draw_minimap(&mut framebuffer, &maze, &mut player, block_size,5);
        }

        if player.win_condition{
            *screen= 3;
            break;
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
            let intersect_f = cast_ray(&mut framebuffer, &maze, &player, player.a, block_size, false, &goal);
            let intersect_b = cast_ray(&mut framebuffer, &maze, &player, player.a+PI, block_size, false, &goal);
            
            let mut wall_f = false;
            if intersect_f.distance < 8.0{
                wall_f = true;
            }
            let mut wall_b = false;
            if intersect_b.distance < 8.0{
                wall_b = true;
            }
            process_events::process_event(&mut player, &window, wall_f, wall_b);
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

    screen_render::pre_play(&mut val, &mut screen);
    if screen!=0{
        playing(val, &mut screen);
    }
    if screen==3{
        screen_render::post_play(&mut val);
    }
    // testing();
}
