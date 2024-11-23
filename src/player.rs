use bevy::prelude::*;
use rand::Rng;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

const GROUND_TIMER: f32 = 0.5;
const MOVEMENT_SPEED: f32 = 8.0;
const JUMP_SPEED: f32 = 1.2;
const GRAVITY: f32 = -9.81/3.0;
const TERMINAL_VELOCITY: f32 = -54.0/3.0;

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
            // &mut Collider,
        ),
        With<Player>,
    >,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut vertical_movement: Local<f32>,
    mut grounded_timer: Local<f32>,

) {

    
    let (mut player_transform, mut controller, output) = player_q.single_mut();


        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut jump_direction =0.0;

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
            jump_direction = 1.0;
        } 


        
        let delta_time = time.delta_seconds();
        let jump_speed = jump_direction * JUMP_SPEED;
        

        let is_grounded = output.map(|o| o.grounded).unwrap_or(false);
        if is_grounded {
            *grounded_timer = GROUND_TIMER;
            *vertical_movement = 0.0;
        } 

        // If we are grounded we can jump
        if *grounded_timer > 0.0 {
            *grounded_timer -= delta_time;
            // If we jump we clear the grounded tolerance
            if jump_speed > 0.0 {
                *vertical_movement = jump_speed;
                *grounded_timer = 0.0;
            }
        }

        

        // direction.y = 0.0;
        let mut movement =  direction.normalize_or_zero() * MOVEMENT_SPEED * delta_time;
        *vertical_movement += GRAVITY * delta_time * controller.custom_mass.unwrap_or(1.0);
        *vertical_movement = (*vertical_movement).max(TERMINAL_VELOCITY);
        // let mut rng = rand::thread_rng();

        // let jitter_vertical: f32 = rng.gen_range(-0.09..=0.09);
        // *vertical_movement += jitter_vertical;

        movement.y = *vertical_movement;
        // movement.y = jump_speed;
        
        
        controller.translation = Some(movement);
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
            transform: Transform::from_xyz(0.0, 0.8, 0.0),
            ..default()
        },
        Player,
        ThirdPersonCameraTarget,
        Name::new("Player"),
        Collider::capsule_y(0.6, 0.3),
        // RigidBody::Dynamic,
        // RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            // ..KinematicCharacterController::default()
            custom_mass: Some(1.0),
            up: Vec3::Y,
            offset: CharacterLength::Absolute(0.03),
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
            // snap_to_ground: Some(CharacterLength::Absolute(0.5)),
            // snap_to_ground: None,
            ..default()
        },
        
    );
    commands.spawn(player).insert(LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED).with_children(|parent| {
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