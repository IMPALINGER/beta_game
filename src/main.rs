use std::env;

use bevy::prelude::*;

use beta_game::{*, player::PlayerPlugin, state::GameState, world::WorldPlugin, camera::CameraPlugin, resources::ResourcePlugin, enemy::EnemyPlugin};
use bevy::render::texture::{ImageFilterMode, ImageSamplerDescriptor};

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        mag_filter: ImageFilterMode::Nearest,
                        min_filter: ImageFilterMode::Nearest,
                        ..default()
                    },
                }),
        )
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))

        .insert_state(GameState::Loading)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(ResourcePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .insert_resource(Msaa::Off)
        .run();
}