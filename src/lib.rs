pub mod animation;
pub mod background;
pub mod character;
pub mod player;
pub mod sprite;

pub static WIDTH: f32 = 800.;
pub static HEIGHT: f32 = 600.;

pub mod prelude {
    pub use crate::animation::AnimationPlugin;
    pub use crate::background::BackgroundPlugin;
    pub use crate::character::CharacterPlugin;
    pub use crate::player::PlayerPlugin;
    pub use crate::sprite::SpritePlugin;
    pub use crate::HEIGHT;
    pub use crate::WIDTH;
}
