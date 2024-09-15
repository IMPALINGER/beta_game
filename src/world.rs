use bevy::prelude::*;

use bevy_lit::prelude::*;
use bevy_rapier2d::prelude::*;

use rand::prelude::*;

use crate::*;
use crate::{state::GameState, GlobalTextureAtlas};

use crate::player::Player;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, add_lights),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default());
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Check if image and layout are Some
    if let (Some(image), Some(layout)) = (&handle.image, &handle.layout) {
        commands
            .spawn((
                SpriteBundle {
                    texture: image.clone(),
                    transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..default()
                },
                TextureAtlas {
                    layout: layout.clone(),
                    index: 0,
                },
                Player,
                KinematicCharacterController {
                    custom_mass: Some(10.0),
                    ..default()
                },

            ))
            .insert(Collider::ball(8.0))
            .insert(Name::new("Player"));
        next_state.set(GameState::InGame);
    } else {
        // Log which field is None for debugging
        if handle.image.is_none() {
            println!("Image is None");
        }
        if handle.layout.is_none() {
            println!("Layout is None");
        }
    }
    commands
        .spawn((Collider::cuboid(50.0, 50.0), LightOccluder2dBundle {
            light_occluder: LightOccluder2d::new(Vec2::new(80.0, 80.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }))
        .insert(Name::new("SAD"))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}


fn add_lights(mut commands: Commands) {
    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            color: Color::srgb(1.0, 1.0, 1.0),
            intensity: 3.0,
            radius: 2000.0,
            falloff: 2.0,
        },
        ..default()
    });// Add lights.
}
