use crate::framebuffer;
use crate::player::Player;
use crate::ray_caster::cast_ray;

pub fn render3d(
    framebuffer: &mut framebuffer::Framebuffer,
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
        if intersect.impact=='g'{
            framebuffer.set_current_color(0x03fc0f);
        } else {
            framebuffer.set_current_color(0xff1100);
        }
        let hh =(framebuffer.height/2) as f32;
        let stake_height = (hh as f32/d_to_wall)*d_to_plane;
        let stake_top = (hh +(stake_height/2.0)) as usize;
        let stake_bottom = (hh -(stake_height/2.0)) as usize;
        for y in stake_bottom..stake_top{
                framebuffer.point(i, y);
        }
    }
    framebuffer.set_current_color(0xffffff);
}
