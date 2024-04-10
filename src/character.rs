use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Debug, Component)]
pub struct Character {
    pub health: f32,
    pub max_health: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub level: u32,
    pub experience: u32,
    pub experience_to_next_level: u32,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            health: 100.,
            max_health: 100.,
            attack: 10.,
            defense: 10.,
            speed: 10.,
            level: 1,
            experience: 0,
            experience_to_next_level: 100,
        }
    }
}

#[derive(Component)]
pub struct CharacterName(pub String);
