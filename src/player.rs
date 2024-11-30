use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

const MOVEMENT_SPEED: f32 = 8.0;
const JUMP_SPEED: f32 = 10.8;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, display_events))
            // .add_systems(Update, (read_character_controller_collisions, debug_player_hit))
            ;
    }
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Grounded(bool);

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &mut Velocity), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
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
    let movement = direction.normalize_or_zero() * MOVEMENT_SPEED * delta_time;

    player_transform.translation += movement;

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
        Grounded(false),
        // HitStatus {is_hit: false, normal1_of_hit: None}
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
        .insert(ActiveEvents::COLLISION_EVENTS)
        .with_children(|parent| {
            parent.spawn(flashlight);
        });
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,

    mut player_q: Query<(Entity, &Collider, &mut Grounded), With<Player>>,
) {
    let (player_id, _player_collider, mut _player_grounded) = player_q.single_mut();
    for collision_event in collision_events.read() {
        // let entity1 = collision_event.0;
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                println!("Collision started between {:?} and {:?}", entity1, entity2);
                // rapier_context.collider_entity(entity1);
                println!("player_id is entity 1: {}", player_id == *entity1);
                println!("player_id is entity 2: {}", player_id == *entity2);
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                println!("Collision stopped between {:?} and {:?}", entity1, entity2);
                // println!("player_id: {player_id:?}");
            }
        }
    }

    // println!("Received collision event: {:?}", collision_event);
    
    // println!("player_collider: {player_collider:?}");
    // if collision with ground started then player is grounded
    // if collsions with ground stopped then player is not grouunded
}

// for contact_force_event in contact_force_events.read() {
//     println!("Received contact force event: {:?}", contact_force_event);
// }
