use bevy::prelude::*;
use bevy_lit::prelude::{AmbientLight2d, Lighting2dPlugin, Lighting2dSettings};
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::*;
use crate::{player::Player, state::GameState};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PanCamPlugin::default(), Lighting2dPlugin))
            .add_systems(
                OnEnter(GameState::GameInit),
                setup_camera,
            )
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::InGame)),
            );
        //.add_systems(OnEnter(GameState::InGame), camera_follow_player)
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(), Lighting2dSettings {
            blur: 32.0,
            ..default()
    },
        AmbientLight2d {
            brightness: 0.36,
            color: Color::Srgba(Srgba::hex("#FFFFFF").unwrap()),
    },)
    ).insert(PanCam {
        grab_buttons: vec![],
        ..default()
    });
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut query_cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if player_query.is_empty() || query_cameras.is_empty() {
        return;
    }

    let player_transform = player_query.single().translation;

    // Update all sprite cameras.
    for mut camera_transform in query_cameras.iter_mut() {
        camera_transform.translation = camera_transform.translation.lerp(player_transform, 0.1)
    }
}
