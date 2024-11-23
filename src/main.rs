use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_physics)
        // .add_systems(Update, print_ball_altitude)
        .run();
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

// fn print_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
//     for mut transform in positions.iter_mut() {
//         dbg!(transform.rotation.to_axis_angle());
//         transform.rotation = Quat::from_rotation_z(270_f32.to_radians());
//         //println!("Ball altitude: {}", transform.translation.y);
//     }
// }