use std::option;

use raylib::prelude::*;

#[derive (PartialEq)]
pub enum PLAYER_STATE{

    IDLE_UP,
    IDLE_DOWN,
    IDLE_RIGHT,
    IDLE_LEFT,

    MOVING_UP,
    MOVING_DOWN,
    MOVING_RIGHT,
    MOVING_LEFT,

}


pub struct Player{

    pub player_front :    Texture2D,
    pub player_back  :    Texture2D,
    pub player_right :    Texture2D,
    pub player_left  :    Texture2D,
    pub player_pos   :      Vector2,
    pub player_state : PLAYER_STATE,
    pub framerec     :    Rectangle,
    pub animation    :         bool,
}

impl  Player {
    pub fn new( rl : &mut RaylibHandle , thread : RaylibThread) -> Player {

	
	let tmp_player_front = rl.load_texture(&thread ,
							     "C:/dev/assets/abigail_front.png")
	    .expect("could not load texture");
	tmp_player_front.set_texture_filter(&thread , TextureFilter::TEXTURE_FILTER_POINT);
	
	let tmp_player_back = rl.load_texture(&thread ,
							     "C:/dev/assets/abigail_back.png")
	    .expect("could not load texture");
	tmp_player_back.set_texture_filter(&thread , TextureFilter::TEXTURE_FILTER_POINT);
	
	let tmp_player_left = rl.load_texture(&thread ,
							     "C:/dev/assets/abigail_left.png")
	    .expect("could not load texture");
	tmp_player_left.set_texture_filter(&thread , TextureFilter::TEXTURE_FILTER_POINT);
	
	let tmp_player_right = rl.load_texture(&thread ,
							     "C:/dev/assets/abigail_right.png")
	    .expect("could not load texture");
	tmp_player_right.set_texture_filter(&thread , TextureFilter::TEXTURE_FILTER_POINT);
	
	Player {
	    player_front : tmp_player_front,
	    player_back : tmp_player_back,
	    player_left : tmp_player_left,
	    player_right : tmp_player_right,

	    player_pos :  Vector2{
		x : 0.0,
		y : 0.0,
	    },
	    player_state : PLAYER_STATE::IDLE_DOWN,

	    framerec : Rectangle{
		x : 0.0,
		y : 0.0,
		width : 16.0,
		height : 27.0,
	    },
	    animation : false,
	    
	}

	
    }

    pub fn animate_player(&mut self ){
	self.framerec.x += 64.0 ;
    }
    
    pub fn  scale_textures(&mut self , factor : i32 ,rl : &mut RaylibHandle , thread : RaylibThread ){

	self.framerec.height = self.framerec.height * (factor as f32);
	self.framerec.width = self.framerec.width * (factor as f32);

	let mut tmp_up = self.player_back.load_image().unwrap();
	tmp_up.resize_nn(tmp_up.width* factor , tmp_up.height * factor);
	self.player_back =  rl.load_texture_from_image(&thread , &tmp_up).unwrap();
	self.player_back.set_texture_filter(&thread , TextureFilter::TEXTURE_FILTER_BILINEAR);
	
	
	let mut tmp_down = self.player_front.load_image().unwrap();
	tmp_down.resize_nn(tmp_down.width* factor , tmp_down.height * factor);
	self.player_front = rl.load_texture_from_image(&thread ,&tmp_down).unwrap();
	self.player_front.set_texture_filter(&thread , TextureFilter::TEXTURE_FILTER_BILINEAR);
	
	let mut tmp_left = self.player_left.load_image().unwrap();
	tmp_left.resize_nn(tmp_left.width* factor , tmp_left.height * factor);
	self.player_left = rl.load_texture_from_image(&thread ,&tmp_left).unwrap();
	self.player_left.set_texture_filter(&thread , TextureFilter::TEXTURE_FILTER_BILINEAR);
	
	let mut tmp_right = self.player_right.load_image().unwrap();
	tmp_right.resize_nn(tmp_right.width* factor , tmp_right.height * factor);
	self.player_right = rl.load_texture_from_image(&thread , &tmp_right).unwrap();
	self.player_right.set_texture_filter(&thread , TextureFilter::TEXTURE_FILTER_BILINEAR);
    }
}
