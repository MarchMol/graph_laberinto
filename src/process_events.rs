use crate::player::Player;
use crate::Window;
use minifb::Key;

pub fn process_event(player: &mut Player, window: &Window, wall_f: bool, wall_b: bool ){

    if window.is_key_down(Key::Left){
        player.inc_a(false)
    }
    if window.is_key_down(Key::Right){
        player.inc_a(true)
    }
    if window.is_key_down(Key::Up){
        if !wall_f{
            player.inc_pos(true)   
        }
    }
    if window.is_key_down(Key::Down){
        if !wall_b{
            player.inc_pos(false)
        }
    }
}