use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

const GROUND_TIMER: f32 = 0.5;
const MOVEMENT_SPEED: f32 = 8.0;
// const JUMP_SPEED: f32 = 1.0;
const GRAVITY: f32 = -9.81/4.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, player_movement)
            // .add_systems(Update, read_result_system)
            ;
    }
}

#[derive(Component)]
struct Player;


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
    mut vertical_movement: Local<f32>,
    // mut grounded_timer: Local<f32>,

) {

    
    for (mut player_transform, mut controller, output) in player_q.iter_mut() {


        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };


        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }

        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }

        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }

        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }

        
        let delta_time = time.delta_seconds();

        // let jump_speed = 1.0 * JUMP_SPEED;
        let is_grounded = output.map(|o| o.grounded).unwrap_or(false);
        // let is_grounded = output.grounded;
        // println!("is grounded: {is_grounded}");
        if is_grounded {
            // *grounded_timer = GROUND_TIMER;
            *vertical_movement = 0.0;
        } else {
            // *vertical_movement = GRAVITY * delta_time * controller.custom_mass.unwrap_or(1.0);
            *vertical_movement = -1.0 * delta_time ;
        }
        // if *grounded_timer > 0.0 {
        //     *grounded_timer -= delta_time;
        //     // If we jump we clear the grounded tolerance
        //     if jump_speed > 0.0 {
        //         *vertical_movement = jump_speed;
        //         *grounded_timer = 0.0;
        //     }
        // }

        
        direction.y = 0.0;
        let mut movement =  direction.normalize_or_zero() * MOVEMENT_SPEED * delta_time;
        movement.y = *vertical_movement;
        
        controller.translation = Some(movement);
        // controller.translation = Some(Vec3::new(0.0,*vertical_movement, 0.0));
        // player_transform.translation += movement;

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
            transform: Transform::from_xyz(0.0, 2.8, 0.0),
            ..default()
        },
        Player,
        ThirdPersonCameraTarget,
        Name::new("Player"),
        Collider::round_cylinder(0.6, 0.3, 0.2),
        // RigidBody::Dynamic,
        // RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            ..KinematicCharacterController::default()
            // custom_mass: Some(5.0),
            // up: Vec3::Y,
            // offset: CharacterLength::Absolute(0.01),
            // slide: true,
            // autostep: Some(CharacterAutostep {
            //     max_height: CharacterLength::Relative(0.3),
            //     min_width: CharacterLength::Relative(0.5),
            //     include_dynamic_bodies: false,
            // }),
            // Don’t allow climbing slopes larger than 45 degrees.
            // max_slope_climb_angle: 45.0_f32.to_radians(),
            // Automatically slide down on slopes smaller than 30 degrees.
            // min_slope_slide_angle: 30.0_f32.to_radians(),
            // apply_impulse_to_dynamic_bodies: true,
            // snap_to_ground: Some(CharacterLength::Absolute(0.5)),
            // snap_to_ground: None,
            // ..default()
        },
    );
    commands.spawn(player).with_children(|parent| {
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