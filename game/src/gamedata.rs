use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}, text::RSliceGlyphInfo, RaylibHandle};

use crate::loadasset::{Player, PLAYER_STATE};

pub struct  Game_Data {

    player : Player ,

    //animation data
    elapsed_time : f32,
    delta_time : f32,
    animation_fps : f32,

    player_speed : f32,
    
}

impl Game_Data{
    pub fn new(p : Player) -> Self {

	Self{
	    player : p,
	    elapsed_time : 0.0,
	    delta_time : 0.0,
	    animation_fps : 5.0,
	    player_speed : 100.0,
	}
	
    }

    pub fn get_delta_time(&mut self , d :&mut RaylibDrawHandle){
	self.delta_time = d.get_frame_time();
    }
    
    pub fn draw_player(&mut self , d :&mut RaylibDrawHandle){

	self.elapsed_time += self.delta_time;

	

	if self.elapsed_time > (1.0/ self.animation_fps){

	    if self.player.animation == true{
		self.player.animate_player();
	    }
	    
	    self.elapsed_time = 0.0;
	
	}

	
	
	if self.player.player_state == PLAYER_STATE::MOVING_UP{
	    d.draw_texture_rec(&self.player.player_back, self.player.framerec , self.player.player_pos , Color::WHITE );
	}

	
	if self.player.player_state == PLAYER_STATE::MOVING_DOWN{
	    d.draw_texture_rec(&self.player.player_front, self.player.framerec , self.player.player_pos , Color::WHITE );
	}

	
	if self.player.player_state == PLAYER_STATE::MOVING_LEFT{
	    d.draw_texture_rec(&self.player.player_left, self.player.framerec , self.player.player_pos , Color::WHITE );
	}

	
	if self.player.player_state == PLAYER_STATE::MOVING_RIGHT{
	    d.draw_texture_rec(&self.player.player_right, self.player.framerec , self.player.player_pos , Color::WHITE );
	}
    }

    pub fn handle_player_input(&mut self , d :&mut RaylibDrawHandle){

	if d.is_key_down(raylib::ffi::KeyboardKey::KEY_W){
	    self.player.player_state = PLAYER_STATE::MOVING_UP;
	    self.player.player_pos.y -= self.player_speed * self.delta_time;
	    self.player.animation = true;

	}else if d.is_key_down(raylib::ffi::KeyboardKey::KEY_S){

	    self.player.player_state = PLAYER_STATE::MOVING_DOWN;
	    self.player.player_pos.y += self.player_speed * self.delta_time;
	    self.player.animation = true;
	}else if d.is_key_down(raylib::ffi::KeyboardKey::KEY_A){

	    self.player.player_state = PLAYER_STATE::MOVING_LEFT;
	    self.player.player_pos.x -= self.player_speed * self.delta_time;
	    self.player.animation = true;
	}else if d.is_key_down(raylib::ffi::KeyboardKey::KEY_D){

	    self.player.player_state = PLAYER_STATE::MOVING_RIGHT;
	    self.player.player_pos.x += self.player_speed * self.delta_time;
	    self.player.animation = true;
	}else{
	    self.player.animation = false;
	}

	
	
    }
}

