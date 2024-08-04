use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
)-> Intersect{
    let mut d = 0.0;
    framebuffer.set_current_color(0xff1100);

    loop{
        let cos = d*a.cos();
        let sin = d*a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x/block_size;
        let j = y/block_size;

        if (maze[i][j] != ' ') & (maze[i][j] != 'p'){

            return Intersect{
                distance: d,
                impact: maze[i][j]
            };
        }
        if draw_line{
            framebuffer.point(x, y);
            d+=10.0;
        }else{
            d+=0.5;
        }
    }
}