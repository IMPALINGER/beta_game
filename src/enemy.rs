use bevy::{prelude::*, time::common_conditions::on_timer};

use bevy::math::{vec3, NormedVectorSpace, VectorSpace};
use bevy::utils::Duration;
use rand::Rng;
use std::f32::consts::PI;

use crate::player::Player;
use crate::*;
use crate::{state::GameState, GlobalTextureAtlas};

use bevy_rapier2d::prelude::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub enum EnemyType {
    Green,
    Red,
    Skin,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_enemy, update_enemy_transform).run_if(in_state(GameState::InGame)),
        );
    }
}

fn spawn_enemy(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    handle: Res<GlobalTextureAtlas>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(SPAWN_RATE_PER_SECOND);

    if num_enemies >= MAX_NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    for _ in 0..enemy_spawn_count {
        let (x, y) = get_random_position_around(player_pos);
        let enemy_type = EnemyType::get_rand_enemy();
        if let (Some(image), Some(layout)) = (&handle.image, &handle.layout) {
            commands
                .spawn((
                    SpriteBundle {
                        texture: image.clone(),
                        transform: Transform::from_translation(vec3(x, y, 1.0))
                            .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                        ..default()
                    },
                    TextureAtlas {
                        layout: layout.clone(),
                        index: 0,
                    },
                    Enemy,
                    KinematicCharacterController {
                        custom_mass: Some(10.0),
                        ..default()
                    },
                ))
                .insert(Collider::cuboid(8.0, 8.0))
                .insert(Name::new("Enemy"))
                //.insert(TransformBundle::from(Transform::from_xyz(
                //    x, y,1.0,)))
                    ;
        } else {
            // Log which field is None for debugging
            if handle.image.is_none() {
                println!("Image is None");
            }
            if handle.layout.is_none() {
                println!("Layout is None");
            }
        }
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(1000.0..5000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}

fn update_enemy_transform(
    player_query: Query<&KinematicCharacterController, With<Player>>,
    mut controllers: Query<&mut KinematicCharacterController, (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || controllers.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;

    for mut controller in controllers.iter_mut() {
        //let dir = (Vec2::ZERO - controller.translation.ok_or_else().normalize();
        controller.translation = Some(Vec2::ONE)
    }
}


impl EnemyType {
    fn get_rand_enemy() -> Self {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0..3);
        return match rand_index {
            0 => Self::Green,
            1 => Self::Red,
            _ => Self::Skin,
        };
    }

    pub fn get_base_sprite_index(&self) -> usize {
        match self {
            EnemyType::Green => 8,
            EnemyType::Red => 12,
            EnemyType::Skin => 20,
        }
    }
}
