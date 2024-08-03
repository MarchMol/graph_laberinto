use crate::player::Player;
use crate::Window;
use minifb::Key;
use core::f32::consts::PI;

pub fn processEvent(player: &mut Player, window: &Window, wall: bool ){

    if window.is_key_down(Key::Left){
        player.incA(false)
    }
    if window.is_key_down(Key::Right){
        player.incA(true)
    }
    if window.is_key_down(Key::Up){
        if !wall{
            player.incPos(true)   
        }
    }
    if window.is_key_down(Key::Down){
        player.incPos(false)
    }
}