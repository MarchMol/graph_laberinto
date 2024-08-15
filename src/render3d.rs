use crate::color;
use crate::color::Color;
use crate::player::Player;
use crate::ray_caster::cast_ray;
use crate::sprite_loader::Sprite;
use crate::Goal;
use crate::framebuffer;

pub fn render3d(
    framebuffer: &mut framebuffer::Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    block_size: usize,
    sprites: &[&Sprite],
    goal: &mut Goal,
) {
    framebuffer.clear();
    let num_rays = framebuffer.width;
    let hh = (framebuffer.height / 2) as f32;
    let mut try_sprite = false;
    let mut sprite_center = 0;
    let mut sprite_distance = 0.0;
    let background_color = color::Color::from_hex(0x323638);
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, maze, player, a, block_size, false, goal);

        if intersect.in_goal{
            try_sprite = true;
            sprite_center = i;
            sprite_distance = intersect.d_to_goal;
        }
        let mut sprite_index = 2;
        if intersect.impact=='+'{
            sprite_index = 0;
        } else if intersect.impact=='-'{
            sprite_index = 1;
        }

        let d_to_wall = if intersect.distance > 10.0 {
            intersect.distance
        } else {
            10.0
        };
        let d_to_plane: f32 = block_size as f32;

        let current_line = sprites[sprite_index].get_line((intersect.texture_index * sprites[sprite_index].width as f32) as usize);

        let stake_height = ((hh + block_size as f32) as f32 / d_to_wall) * d_to_plane;
        let stake_top = (hh + (stake_height / 2.0)) as usize;
        let stake_bottom = (hh - (stake_height / 2.0)) as usize;

        for y in 0..framebuffer.height {
            if (y > stake_bottom) & (y < stake_top) {
                let trans_y =
                    (sprites[sprite_index].height as f32) * (y as f32 - hh + (stake_height / 2.0)) / stake_height;
                framebuffer.set_current_color(Color::to_hex(&current_line[trans_y as usize]));
                framebuffer.point(i, y);
            } else if y <= stake_bottom {
                framebuffer.set_current_color(Color::to_hex(&(background_color*(1.5-(y as f32/hh)))));
            } else {
                framebuffer.set_current_color(Color::to_hex(&(background_color*(-0.5+(y as f32/hh)))));
            }
            framebuffer.point(i, y);
        }
    }
    if try_sprite & (sprite_distance>10.0){
        draw_sprite(framebuffer, block_size, goal, sprite_distance, sprite_center)
    } else if try_sprite & (sprite_distance<10.0){
        player.win_condition=true;
    }
    framebuffer.set_current_color(0xffffff);
}

pub fn draw_sprite(
    framebuffer: &mut framebuffer::Framebuffer,
    block_size: usize,
    goal: &mut Goal,
    sprite_distance: f32,
    sprite_center: usize
){

        let hh = (framebuffer.height / 2) as f32;
        let sprite_height = ((hh/2.0 + block_size as f32) as f32 / sprite_distance) * block_size as f32;
        let draw_start_y = (hh - (sprite_height / 2.0)) as usize;
        let draw_end_y = (hh + (sprite_height / 2.0)) as usize;

        let draw_start_x = -sprite_height as i32/2 +sprite_center as i32;
        let mut draw_end_x = sprite_height as i32/2 + sprite_center as i32;
        if draw_end_x >= framebuffer.width as i32 {draw_end_x = framebuffer.width as i32- 1};

        for x in draw_start_x..draw_end_x{
            if x >= 0{
                let trans_x = ((x as f32-draw_start_x as f32)/sprite_height as f32)*(goal.sprite.height) as f32;
                let current_line = goal.sprite.get_line(trans_x as usize);
                for y in draw_start_y..draw_end_y{
                    let trans_y =
                    (goal.sprite.height as f32) * (y as f32 - hh + (sprite_height / 2.0)) / sprite_height;
                    let color = Color::to_hex(&current_line[trans_y as usize]);
                    if color!=0xFFFFFF{
                        framebuffer.set_current_color(color);
                        framebuffer.point(x as usize, y);
                    }

                }
            }
        }
    
}