use raylib::prelude::*;

use crate::gamedata::Game_Data;


#[derive(PartialEq)]
pub enum GAME_STATE{
    INGAME,
    INMENU,
}


pub fn draw_in_game(d : &mut RaylibDrawHandle , g : &mut Game_Data ){
    
    d.clear_background(Color::WHITE);
    g.get_delta_time( d);
    g.draw_player( d);
    g.handle_player_input(d);
}


pub fn draw_in_menu(d : &mut RaylibDrawHandle , g : &mut Game_Data){
    
    d.clear_background(Color::BLACK);
    d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);
}
