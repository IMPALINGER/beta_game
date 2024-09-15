use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;
use enemy::Enemy;

use crate::state::GameState;
use crate::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_player_input.run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_input(
    mut controllers: Query<&mut KinematicCharacterController, (With<Player>, Without<Enemy>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if controllers.is_empty() {
        return;
    }

    let mut delta: Vec2 = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        delta.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        delta.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        delta.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        delta.x += 1.0;
    }
    delta = delta.normalize();

    for mut controller in controllers.iter_mut() {
        controller.translation = Some(delta)
    }

    //if delta.is_finite() && delta != Vec2::ZERO {
    //    transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
    //}
}
