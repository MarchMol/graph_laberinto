use minifb::Window;
use nalgebra_glm::{pi, Vec2};
use core::f32::consts::PI;

const MOVE_SPEED: f32 = 5.0;
const ROTATION_SPEED: f32 = PI/50.0;

pub struct Player{
    pub pos:nalgebra_glm::Vec2,
    pub a: f32,
    pub fov: f32
}

impl Player{
    pub fn new()->Self{
        Player{
            pos: Vec2::new(0.0, 0.0),
            a: PI/3.0,
            fov: PI/3.0
        }
    }

    pub fn setPos(&mut self, x: f32, y: f32){
        self.pos.x = x;
        self.pos.y = y;
    }
    pub fn incA(&mut self, orientation: bool){ // true - Right, false - left
        if orientation{
            self.a += ROTATION_SPEED;
        } else {
            self.a -= ROTATION_SPEED;
        }
    }
    pub fn incPos(&mut self, orientation: bool){ // true - front, false - back
        let cos = MOVE_SPEED*self.a.cos();
        let sin = MOVE_SPEED*self.a.sin();
        if orientation{
            self.pos.x += cos;
            self.pos.y += sin;
        } else {
            self.pos.x -= cos;
            self.pos.y -= sin;
        }
    }
}