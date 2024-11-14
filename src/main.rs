use bevy::prelude::*;


pub use bevy_third_person_camera::ThirdPersonCameraPlugin;

mod camera;
mod player;
mod world;


use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ThirdPersonCameraPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}
