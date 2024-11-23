use bevy::{input::InputSystem, prelude::*};

use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

// const GROUND_TIMER: f32 = 0.5;
const MOVEMENT_SPEED: f32 = 8.0;
// const JUMP_SPEED: f32 = 20.0;
// const GRAVITY: f32 = -9.81;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementInput>()
            .add_systems(Startup, spawn_player)
            // .add_systems(PreUpdate, handle_input.after(InputSystem))
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

/// Keyboard input vector
#[derive(Default, Resource, Deref, DerefMut)]
struct MovementInput(Vec3);

// fn handle_input(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut movement: ResMut<MovementInput>,
//     cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
// ) {
//     let cam = match cam_q.get_single() {
//         Ok(c) => c,
//         Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
//     };

//     let mut direction = Vec3::ZERO;
//     if keyboard.pressed(KeyCode::KeyW) {
//         direction -= *cam.forward();
//     }
//     if keyboard.pressed(KeyCode::KeyS) {
//         direction += *cam.back();
//     }
//     if keyboard.pressed(KeyCode::KeyA) {
//         direction -= *cam.left();
//     }
//     if keyboard.pressed(KeyCode::KeyD) {
//         direction += *cam.right();
//     }
//     **movement = direction.normalize_or_zero();
//     if keyboard.pressed(KeyCode::ShiftLeft) {
//         **movement *= 2.0;
//     }

// }

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<
        (
            &mut Transform,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,

) {

    

    for (mut player_transform, mut _controller, _output) in player_q.iter_mut() {


        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };


        let mut direction = Vec3::ZERO;

        //forward
        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }

        //back
        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }

        //left
        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }

        //back
        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }

        direction.y = 0.0;
        let delta_time = time.delta_seconds();
        // Retrieve input
     
        let movement =  direction.normalize_or_zero() * MOVEMENT_SPEED * delta_time;
  

        player_transform.translation += movement;

        //
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y)
        }
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    //216, 224, 69, 255
    let flashlight = (
        SpotLightBundle {
            spot_light: SpotLight {
                color: Srgba::rgba_u8(216, 224, 69, 255).into(),
                outer_angle: 0.6,
                inner_angle: 0.5,
                intensity: 1000000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -0.5),

            ..default()
        },
        Name::new("Flashlight"),
    );
    let player = (
        SceneBundle {
            scene: assets.load("Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.8, 0.0),
            ..default()
        },
        Player,
        ThirdPersonCameraTarget,
        Name::new("Player"),
        Collider::round_cylinder(0.6, 0.3, 0.2),
        // RigidBody::Dynamic,
        KinematicCharacterController {
            custom_mass: Some(5.0),
            up: Vec3::Y,
            offset: CharacterLength::Absolute(0.01),
            slide: true,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(0.3),
                min_width: CharacterLength::Relative(0.5),
                include_dynamic_bodies: false,
            }),
            // Don’t allow climbing slopes larger than 45 degrees.
            max_slope_climb_angle: 45.0_f32.to_radians(),
            // Automatically slide down on slopes smaller than 30 degrees.
            min_slope_slide_angle: 30.0_f32.to_radians(),
            apply_impulse_to_dynamic_bodies: true,
            snap_to_ground: None,
            ..default()
        },
    );
    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
}
