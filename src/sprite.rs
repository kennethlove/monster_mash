use crate::animation::{AnimationIndices, AnimationTimer};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<DirectionalSpriteAction>::default())
            .add_event::<TurnEvent>()
            .add_systems(Startup, spawn_sprites)
            .add_systems(FixedUpdate, turn_sprite)
            .add_systems(Update, update_directional_sprite);
    }
}

#[derive(Debug, Event)]
enum TurnEvent {
    Turn(Direction2d),
}

#[derive(Component)]
pub struct DirectionalSprite {
    pub directions: Vec<SpriteDirectionIndex>,
}

#[derive(Clone, Component, Debug)]
pub struct SpriteDirectionIndex {
    pub direction: SpriteDirection,
    pub indices: AnimationIndices,
}

#[derive(Clone, Component, Debug, PartialEq)]
pub enum SpriteDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum DirectionalSpriteAction {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Actions
}

impl DirectionalSpriteAction {
    const DIRECTIONS: [Self; 4] = [
        DirectionalSpriteAction::Up,
        DirectionalSpriteAction::Down,
        DirectionalSpriteAction::Left,
        DirectionalSpriteAction::Right,
    ];

    fn direction(self) -> Option<Direction2d> {
        match self {
            DirectionalSpriteAction::Up => Some(Direction2d::Y),
            DirectionalSpriteAction::Down => Some(Direction2d::NEG_Y),
            DirectionalSpriteAction::Left => Some(Direction2d::NEG_X),
            DirectionalSpriteAction::Right => Some(Direction2d::X),
            _ => None,
        }
    }
}

#[derive(Bundle)]
struct DirectionalSpriteBundle {
    directional_sprite: DirectionalSprite,
    animation_timer: AnimationTimer,
    sprite_sheet: SpriteSheetBundle,
    input_manager: InputManagerBundle<DirectionalSpriteAction>,
    animation_indices: AnimationIndices,
    facing_direction: SpriteDirection,
}

const SPRITE_DIRECTIONS: [SpriteDirectionIndex; 4] = [
    SpriteDirectionIndex {
        direction: SpriteDirection::Up,
        indices: AnimationIndices {
            first: 36,
            last: 38,
        },
    },
    SpriteDirectionIndex {
        direction: SpriteDirection::Down,
        indices: AnimationIndices { first: 0, last: 2 },
    },
    SpriteDirectionIndex {
        direction: SpriteDirection::Left,
        indices: AnimationIndices {
            first: 12,
            last: 14,
        },
    },
    SpriteDirectionIndex {
        direction: SpriteDirection::Right,
        indices: AnimationIndices {
            first: 24,
            last: 26,
        },
    },
];

impl Default for DirectionalSpriteBundle {
    fn default() -> Self {
        Self {
            directional_sprite: DirectionalSprite { directions: vec![] },
            animation_timer: AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            sprite_sheet: SpriteSheetBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Vec2::new(64., 64.).into(),
                    ..default()
                },
                ..default()
            },
            input_manager: InputManagerBundle::default(),
            animation_indices: AnimationIndices { first: 0, last: 0 },
            facing_direction: SpriteDirection::Down,
        }
    }
}

impl DirectionalSpriteBundle {
    fn new(
        directional_sprite: DirectionalSprite,
        animation_timer: AnimationTimer,
        sprite_sheet: SpriteSheetBundle,
        animation_indices: AnimationIndices,
        facing_direction: Option<SpriteDirection>,
    ) -> Self {
        let facing_direction = facing_direction.unwrap_or(SpriteDirection::Down);
        Self {
            directional_sprite,
            animation_timer,
            animation_indices,
            sprite_sheet,
            input_manager: InputManagerBundle::with_map(
                DirectionalSpriteBundle::default_input_map(),
            ),
            facing_direction,
        }
    }

    fn default_input_map() -> InputMap<DirectionalSpriteAction> {
        use DirectionalSpriteAction::*;
        let mut input_map = InputMap::default();

        // Direction and Movement
        input_map.insert(Up, KeyCode::ArrowUp);
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Up, GamepadButtonType::DPadUp);

        input_map.insert(Down, KeyCode::ArrowDown);
        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Down, GamepadButtonType::DPadDown);

        input_map.insert(Left, KeyCode::ArrowLeft);
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Left, GamepadButtonType::DPadLeft);

        input_map.insert(Right, KeyCode::ArrowRight);
        input_map.insert(Right, KeyCode::KeyD);
        input_map.insert(Right, GamepadButtonType::DPadRight);

        input_map
    }
}

fn spawn_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("characters-ab.png");
    let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 12, 8, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_timer = AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating));
    let sprite_sheet = SpriteSheetBundle {
        texture: texture_handle,
        atlas: TextureAtlas {
            layout: texture_atlas_handle,
            index: 0,
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Vec2::new(64., 64.).into(),
            ..default()
        },
        ..default()
    };
    let animation_indices = SPRITE_DIRECTIONS.clone()[1].indices.clone();
    let directional_sprite = DirectionalSprite {
        directions: SPRITE_DIRECTIONS.clone().into(),
    };

    let dsb = DirectionalSpriteBundle {
        directional_sprite,
        animation_timer,
        sprite_sheet,
        animation_indices: animation_indices.clone(),
        input_manager: InputManagerBundle::with_map(DirectionalSpriteBundle::default_input_map()),
        ..default()
    };
    commands.spawn(dsb);
}

fn turn_sprite(
    mut commands: Commands,
    mut events: EventReader<TurnEvent>,
    mut query: Query<(Entity, &mut TextureAtlas), With<DirectionalSprite>>,
) {
    for event in events.read() {
        let indices = match event {
            TurnEvent::Turn(direction) => match *direction {
                Direction2d::X => SPRITE_DIRECTIONS[3].indices.clone(), // Right
                Direction2d::NEG_X => SPRITE_DIRECTIONS[2].indices.clone(), // Left
                Direction2d::Y => SPRITE_DIRECTIONS[0].indices.clone(), // Down
                Direction2d::NEG_Y => SPRITE_DIRECTIONS[1].indices.clone(), // Up
                _ => SPRITE_DIRECTIONS[1].indices.clone(),              // Default to down
            },
        };
        for (entity, mut atlas) in &mut query {
            let mut entity = commands.entity(entity);
            entity.insert(indices.clone());
            if atlas.index < indices.first || atlas.index > indices.last {
                atlas.index = indices.first;
            }
        }
    }
}

fn update_directional_sprite(
    query: Query<&ActionState<DirectionalSpriteAction>, With<DirectionalSprite>>,
    mut events: EventWriter<TurnEvent>,
) {
    let action_state = query.single();

    let mut direction_vector = Vec2::ZERO;

    for input_direction in DirectionalSpriteAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                direction_vector += *direction;
            }
        }
    }

    let net_direction = Direction2d::new(direction_vector);

    if let Ok(direction) = net_direction {
        events.send(TurnEvent::Turn(direction));
    }
}
