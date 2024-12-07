use crate::world::{Cubeovator, Ground};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

const MOVEMENT_SPEED: f32 = 8.0;
const JUMP_SPEED: f32 = 22.8;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, grounded_ungrounded_on_collision))
            // .add_systems(Update, (read_character_controller_collisions, debug_player_hit))
            ;
    }
}

#[derive(Component)]
pub struct Player;
#[derive(Component, PartialEq)]
// We also want to track the id of the ground he is touching
// If he is on one ground only and that ground is moving, then we want to apply the translation to the player
pub struct Grounded {
    pub count: u16,
    pub entities: Vec<Entity>,
}

#[derive(Component)]
struct PlayerGroundedSensor;

// Check for Some(Cubeovator)
fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &mut Velocity, &Grounded), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    ground_q: Query<&Parent, With<Ground>>,
    cubeovator_q: Query<&Cubeovator>,
) {
    // let (mut player_transform, mut _controller, _output) = player_q.single_mut();
    let (mut player_transform, mut player_velocity, grounded) = player_q.single_mut();

    let cam = match cam_q.get_single() {
        Ok(c) => c,
        Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
    };

    // let mut jump_direction = 0.0;

    let mut direction = Vec3::ZERO;
    let mut movement_linvel = Vec3::ZERO;
    // We need to remove the y component out of these
    if keys.pressed(KeyCode::KeyW) {
        if grounded.count > 0 {
            direction += (*cam.forward()).with_y(0.0);
        }
    }

    if keys.pressed(KeyCode::KeyS) {
        if grounded.count > 0 {
            direction += (*cam.back()).with_y(0.0);
        }
    }

    if keys.pressed(KeyCode::KeyA) {
        if grounded.count > 0 {
            direction += (*cam.left()).with_y(0.0);
        }
    }

    if keys.pressed(KeyCode::KeyD) {
        if grounded.count > 0 {
            direction += (*cam.right()).with_y(0.0);
        }
    }

    if keys.pressed(KeyCode::KeyE) {
        // jump_direction = 1.0;
        if grounded.count > 0 {
            movement_linvel += Vec3::new(0.0, JUMP_SPEED, 0.0);
        }
    }
    // is it a cubeovator?
    if grounded.count == 1 {
        if let Ok(ground_parent) = ground_q.get(grounded.entities[0]) {
            // if the ground has a parent, then check if it is a cubeovator
            if let Ok(_cubeovator) = cubeovator_q.get(ground_parent.get()) {
                // println!("Riding the Cubeovator!");
                // player_transform.translation.y += cubeovator.oscillator;
            }
        }
    }

    let delta_time = time.delta_seconds();

    let movement = direction.normalize_or_zero() * MOVEMENT_SPEED * delta_time;
    movement_linvel += movement * 5.0;

    player_transform.translation += movement;
    player_velocity.linvel += movement_linvel;

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
        Grounded {
            count: 0,
            entities: vec![],
        },
        // HitStatus {is_hit: false, normal1_of_hit: None}
    );
    commands
        .spawn(player)
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Restitution::coefficient(0.1))
        .insert(GravityScale(5.5))
        .insert(AdditionalMassProperties::Mass(20.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .with_children(|parent| {
            parent.spawn(flashlight);
            // Can we use the sensor collider to provide us some grounded tolerance?
            parent.spawn((PlayerGroundedSensor, Collider::cone(1.0, 0.3))).insert(ActiveEvents::COLLISION_EVENTS).insert(Sensor);
        });
}

// Instead we probably just want to check if the player is in close proximity to the ground.  Less buggy.
// Doo this with a ground sensor
fn grounded_ungrounded_on_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_q: Query<(&mut Grounded, &Children), With<Player>>,
    ground_q: Query<Entity, With<Ground>>,
    player_ground_sensor_collider_q: Query<Entity,With<PlayerGroundedSensor>>,
) {
    let ( mut player_grounded, children) = player_q.single_mut();
    for &child in children.iter() {
        if let Ok(player_ground_sensor_id) = player_ground_sensor_collider_q.get(child) {
            for collision_event in collision_events.read() {
                // let entity1 = collision_event.0;
                match collision_event {
                    CollisionEvent::Started(entity1, entity2, _flags) => {
                        let (some_player_collision_entity, other_entity) = match player_ground_sensor_id {
                            id if id == *entity1 => (Some(entity1), entity2),
                            id if id == *entity2 => (Some(entity2), entity1),
                            _ => (None, entity1),
                        };
                        if let Some(_player_collision_entity) = some_player_collision_entity {
                            let first_ground = ground_q.iter().filter(|&g| g == *other_entity).next();
                            if let Some(_ground) = first_ground {
                                player_grounded.count += 1;
                                player_grounded.entities.push(_ground);
                                // *player_grounded = Grounded(player_grounded.count + 1);
                                println!("New ground, total count: {}", player_grounded.count);
                            }
                        }
                    }
                    CollisionEvent::Stopped(entity1, entity2, _flags) => {
                        let (some_player_collision_entity, other_entity) = match player_ground_sensor_id {
                            id if id == *entity1 => (Some(entity1), entity2),
                            id if id == *entity2 => (Some(entity2), entity1),
                            _ => (None, entity1),
                        };
                        if let Some(_player_collision_entity) = some_player_collision_entity {
                            let first_ground = ground_q.iter().filter(|&g| g == *other_entity).next();
                            if let Some(_ground) = first_ground {
                                player_grounded.count = std::cmp::max(0, player_grounded.count - 1);
                                player_grounded.entities.retain(|&x| x != _ground);
                                println!("Left ground, total count: {}", player_grounded.count);
                            }
                        }
                    }
                }
            }
        }
    }
    //also query the PlayerGroundedSensor
    

    // println!("Received collision event: {:?}", collision_event);

    // println!("player_collider: {player_collider:?}");
    // if collision with ground started then player is grounded
    // if collsions with ground stopped then player is not grouunded
}

// for contact_force_event in contact_force_events.read() {
//     println!("Received contact force event: {:?}", contact_force_event);
// }
