use bevy::prelude::*;
use bevy_rapier3d::prelude::*;


pub use bevy_third_person_camera::ThirdPersonCameraPlugin;

mod camera;
mod player;
mod world;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        // .insert_resource(Time::<Fixed>::from_seconds(0.25))
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ThirdPersonCameraPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        
        // .add_systems(Update, print_ball_altitude)
        .run();
}




