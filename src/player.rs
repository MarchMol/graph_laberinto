use nalgebra_glm::Vec2;
use core::f32::consts::PI;


const ROTATION_SPEED: f32 = PI/50.0;

pub struct Player{
    pub pos:nalgebra_glm::Vec2,
    pub a: f32,
    pub fov: f32,
    pub win_condition: bool,
    move_speed: f32
}

impl Player{
    pub fn new(block_size: usize)->Self{
        Player{
            pos: Vec2::new(0.0, 0.0),
            a: PI/3.0,
            fov: PI/3.0,
            win_condition: false,
            move_speed: block_size as f32/10.0
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32){
        self.pos.x = x;
        self.pos.y = y;
    }
    pub fn inc_a(&mut self, orientation: bool){ // true - Right, false - left
        if orientation{
            self.a += ROTATION_SPEED;
        } else {
            self.a -= ROTATION_SPEED;
        }
    }
    pub fn inc_pos(&mut self, orientation: bool){ // true - front, false - back
        let cos = self.move_speed*self.a.cos();
        let sin = self.move_speed*self.a.sin();
        if orientation{
            self.pos.x += cos;
            self.pos.y += sin;
        } else {
            self.pos.x -= cos;
            self.pos.y -= sin;
        }
    }
}