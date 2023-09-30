use bevy::prelude::*;

pub const IROWS:i32 = 4;
pub const JCOLS:i32 = 4;

#[derive(Component)]
pub struct Position { 
    pub x:i32, 
    pub y:i32 
}

#[derive(Component)]
pub struct Tile {
    pub index:i32
}