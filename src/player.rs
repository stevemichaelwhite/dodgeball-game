use bevy::prelude::*;
// use rand::Rng;
// use crate::world::Ball;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

// const GROUND_TIMER: f32 = 0.5;
const MOVEMENT_SPEED: f32 = 8.0;
const JUMP_SPEED: f32 = 10.8;
// const GRAVITY: f32 = -9.81 / 4.0;
// const TERMINAL_VELOCITY: f32 = -54.0 / 3.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            // .add_systems(Update, (read_character_controller_collisions, debug_player_hit))
            ;
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, Debug)]
struct HitStatus {
    is_hit: bool,
    normal1_of_hit: Option<Vec3>,
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<
        (
            &mut Transform,
            &mut Velocity,
            // &mut KinematicCharacterController,
            // Option<&KinematicCharacterControllerOutput>,
            // &mut Collider,
        )
        ,
        With<Player>,
    >,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    // vertical_movement: Local<f32>,
    // mut grounded_timer: Local<f32>,
) {
    // let (mut player_transform, mut _controller, _output) = player_q.single_mut();
    let (mut player_transform, mut player_velocity) = player_q.single_mut();

    let cam = match cam_q.get_single() {
        Ok(c) => c,
        Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
    };

    // let mut jump_direction = 0.0;

    let mut direction = Vec3::ZERO;
    // We need to remove the y component out of these
    if keys.pressed(KeyCode::KeyW) {
        direction += (*cam.forward()).with_y(0.0);
    }

    if keys.pressed(KeyCode::KeyS) {
        direction += (*cam.back()).with_y(0.0);
    }

    if keys.pressed(KeyCode::KeyA) {
        direction += (*cam.left()).with_y(0.0);
    }

    if keys.pressed(KeyCode::KeyD) {
        direction += (*cam.right()).with_y(0.0);
    }
    if keys.pressed(KeyCode::KeyE) {
        // jump_direction = 1.0;
        player_velocity.linvel = Vec3::new(0.0, JUMP_SPEED, 0.0);
    }

    let delta_time = time.delta_seconds();
    // let jump_speed = jump_direction * JUMP_SPEED;

    // let is_grounded = output.map(|o| o.grounded).unwrap_or(false);
    // if is_grounded {
    //     *grounded_timer = GROUND_TIMER;
    //     *vertical_movement = 0.0;
    // }

    // // If we are grounded we can jump
    // if *grounded_timer > 0.0 {
    //     *grounded_timer -= delta_time;
    //     // If we jump we clear the grounded tolerance
    //     if jump_speed > 0.0 {
    //         *vertical_movement = jump_speed;
    //         *grounded_timer = 0.0;
    //     }
    // }

    // direction.y = 0.0;
    let mut movement = direction.normalize_or_zero() * MOVEMENT_SPEED * delta_time;
    // *vertical_movement += GRAVITY * delta_time * controller.custom_mass.unwrap_or(1.0);
    // *vertical_movement = (*vertical_movement).max(TERMINAL_VELOCITY);


    // movement.y = *vertical_movement;
    // movement.y = jump_speed;

    player_transform.translation += movement;
    // controller.translation = Some(Vec3::new(0.0,*vertical_movement, 0.0));
    // player_transform.translation += movement;

    //
    if direction.length_squared() > 0.0 {
        player_transform.look_to(direction, Vec3::Y)
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
            transform: Transform::from_xyz(0.0, 1.8, 0.0),
            ..default()
        },
        Player,
        ThirdPersonCameraTarget,
        Name::new("Player"),
        Collider::cone(0.8, 0.3),
        RigidBody::Dynamic,
        // RigidBody::KinematicPositionBased,
        // KinematicCharacterController {
        //     // ..KinematicCharacterController::default()
        //     custom_mass: Some(1.0),
        //     up: Vec3::Y,
        //     offset: CharacterLength::Absolute(0.03),
        //     slide: true,
        //     autostep: Some(CharacterAutostep {
        //         max_height: CharacterLength::Relative(0.3),
        //         min_width: CharacterLength::Relative(0.5),
        //         include_dynamic_bodies: false,
        //     }),
        //     // Don’t allow climbing slopes larger than 45 degrees.
        //     max_slope_climb_angle: 45.0_f32.to_radians(),
        //     // Automatically slide down on slopes smaller than 30 degrees.
        //     min_slope_slide_angle: 30.0_f32.to_radians(),
        //     apply_impulse_to_dynamic_bodies: true,
        //     snap_to_ground: Some(CharacterLength::Absolute(0.5)),
        //     // snap_to_ground: None,
        //     ..default()
        // },
        HitStatus {is_hit: false, normal1_of_hit: None}
    );
    commands
        .spawn(player)
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(GravityScale(5.5))
        .insert(AdditionalMassProperties::Mass(20.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        // .insert(ExternalImpulse {
        //     impulse: Vec3::new(50.0, 0.0, 25.0),
        //     torque_impulse: Vec3::new(0.0, 0.0, 0.0),
        // })
        .with_children(|parent| {
            parent.spawn(flashlight);
        });
}

// fn read_result_system(controllers: Query<(&KinematicCharacterControllerOutput)>) {
//     for (output) in controllers.iter() {
//         println!(
//             "Entity touches the ground: {:?}",
//             output.grounded
//         );
//     }
// }

// fn read_character_controller_collisions(
//     mut character_controller_outputs: Query<
//         (&mut KinematicCharacterControllerOutput, &mut HitStatus)>,
//     ball_q: Query<Entity, With<Ball>>,
// ) {
//     for ball_entity in ball_q.iter() {
//         // println!("ball_entity: {ball_entity}");
//         for (output, mut hit_status) in character_controller_outputs.iter_mut() {
//             for collision in &output.collisions {
//                 if ball_entity == collision.entity {
                    
//                     // let hit = collision.hit.details
//                     if let Some(hit_details) = collision.hit.details {
//                         println!("hit details: {:?}", hit_details);
//                         *hit_status = HitStatus{is_hit:true, normal1_of_hit: Some(hit_details.normal1)};
//                     }
                    
//                     // we need to flip was_hit to true and save the normal, and the mass of the object
//                     // body.apply(value);
//                 }
//             }
//         }
//     }
// }

// fn debug_player_hit(
//     mut query: Query<
//         (&HitStatus,
//         &mut KinematicCharacterController),
//         (With<Player>, Changed<HitStatus>)>,
// ) {
//     for (hit_status, mut controller) in query.iter_mut() {
//         eprintln!(
//             "hit_status: {:?}",
//             hit_status
//         );
//         controller.translation = hit_status.normal1_of_hit;
//     }
// }
