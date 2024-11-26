use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f64::consts::TAU;

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
        .add_systems(Startup, (setup_floor, spawn_ball))
        .add_systems(FixedUpdate, move_cubes)
        // .add_systems(Update, print_ball_altitude)
        .run();
}

fn setup_floor(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
}

fn spawn_ball(mut commands: Commands) {
    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        // .insert(AdditionalMassProperties::Mass(0.2))
        .insert(Restitution::coefficient(0.9))
        .insert(TransformBundle::from(Transform::from_xyz(-4.0, 1.0, 0.0)))
        .insert(Friction {
            coefficient: 0.00,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Velocity {
            linvel: Vec3::new(30.0, 1.0, 0.0),
            angvel: Vec3::new(0.2, 0.4, 0.8),
        });
}

fn move_cubes(
    time: Res<Time>,
    mut cube_q: Query<&mut Transform, (With<RigidBody>, With<world::Cube>)>,
) {
    cube_q.iter_mut().for_each(|mut transfrom| {
        let oscillator = (time.elapsed_seconds() % (TAU as f32)).sin();
        transfrom.translation.y += oscillator / 6.0;
        // println!("Sine seconds: {}", oscillator);
    });
}
