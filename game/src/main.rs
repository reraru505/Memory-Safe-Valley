#![allow(
    dead_code,
    unused_variables,
    non_camel_case_types,
    unused_imports,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
    
)]

pub mod loadasset;
pub mod gamedata;
pub mod gamestate;

use level_editor::*; 
use gamedata::Game_Data;
use loadasset::Player;
use raylib::prelude::*;
use crate::gamestate::GAME_STATE;
use crate::gamestate::{draw_in_game , draw_in_menu};

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Hello, World")
        .resizable()
        .build();

    

    let mut player = Player::new(&mut rl, thread.clone());
    player.scale_textures(4 , &mut rl , thread.clone());
    
    let mut gamedata = Game_Data::new(player);
    
    let gamestate : GAME_STATE = GAME_STATE::INGAME; 
    
    while !rl.window_should_close() {
	
        let mut d = rl.begin_drawing(&thread);

	if gamestate == GAME_STATE::INMENU {
	    draw_in_menu(&mut d , &mut gamedata);
	}
	if gamestate == GAME_STATE::INGAME {
	    draw_in_game(&mut d , &mut gamedata );
	}
	
    }
}
