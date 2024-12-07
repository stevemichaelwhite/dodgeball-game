use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f64::consts::TAU;
use std::time::Duration;

pub struct WorldPlugin;

#[derive(Component)]
pub struct Ground;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_light,
                spawn_directional_light,
                spawn_floor,
                spawn_objects,
                setup_floor,
                // spawn_ball,
                setup_ball_spawning,
            ),
        )
        .add_systems(Update, (move_cubes, spawn_ball, despawn_ball));
    }
}

fn spawn_light(mut commands: Commands) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 0.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Name::new("PointLight"),
    );
    commands.spawn(light);
}

fn spawn_directional_light(mut commands: Commands) {
    let light = (
        DirectionalLightBundle {
            transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            directional_light: DirectionalLight {
                illuminance: 1_000.,
                shadows_enabled: true,
                color: Srgba::rgba_u8(137, 123, 43, 255).into(),

                ..default()
            },
            ..default()
        },
        Name::new("DirectionalLight"),
    );
    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_size = 10.0;
    let ground_height = 0.1;
    let floor = (
        PbrBundle {
            mesh: meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(ground_size, ground_size)
                    .subdivisions(10),
            ),
            // mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
            material: materials.add(StandardMaterial {
                base_color: Srgba::hex("#21ad1a").unwrap().into(),
                metallic: 0.620,
                perceptual_roughness: 0.8,
                ..Default::default()
            }),
            ..default()
        },
        Name::new("Floor"),
        Collider::cuboid(ground_size / 2.0, ground_height, ground_size / 2.0),
        // RigidBody::Fixed,
        Ground,
    );
    commands.spawn(floor);
}

#[derive(Component)]

// we need to add the oscillator here so that player movement can query it
pub struct Cubeovator {
    pub oscillator: f32,
}

#[derive(Bundle)]
struct GroundBundle(Collider, Ground, Transform);

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut create_cube = |hdim_xyz: (f32, f32, f32),
                           color_hex: String,
                           xyz: (f32, f32, f32),
                           name: String|
     -> (
        (PbrBundle, Name, Collider, RigidBody, Cubeovator),
        GroundBundle,
    ) {
        let cube = (
            PbrBundle {
                mesh: meshes.add(Cuboid::new(hdim_xyz.0, hdim_xyz.1, hdim_xyz.2)),
                material: materials.add(StandardMaterial {
                    base_color: Srgba::hex(color_hex).unwrap().into(),
                    metallic: 0.620,
                    perceptual_roughness: 0.8,
                    ..default()
                }),
                transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
                ..default()
            },
            Name::new(name),
            Collider::cuboid(hdim_xyz.0 / 2.0, hdim_xyz.1 / 2.0, hdim_xyz.2 / 2.0),
            RigidBody::KinematicPositionBased,
            Cubeovator { oscillator: 0.0 },
            // Ground,
        );
        let cube_ground = GroundBundle(
            Collider::cuboid(hdim_xyz.0 / 2.0 - 0.05, 0.5, hdim_xyz.2 / 2.0 - 0.05),
            Ground,
            // translate by halfy cube - halfy ground
            Transform::from_xyz(0.0, hdim_xyz.1 / 2.0 - 0.5 + 0.05, 0.0),
        );
        (cube, cube_ground)
    };
    let (blue_cube, blue_cube_ground) = create_cube(
        (2.0, 2.0, 2.0),
        "#1a1fad".to_string(),
        (1.7, 1.0, 0.0),
        "BlueCubeovator".to_string(),
    );
    commands.spawn(blue_cube).with_children(|parent| {
        parent.spawn(blue_cube_ground);
    });

    // commands.spawn(create_cube(
    //     (2.0, 2.0, 2.0),
    //     "#ad1a30".to_string(),
    //     (-3.3, 1.0, 3.5),
    //     "RedCubeovator".to_string(),
    // ));
}

fn setup_floor(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn((Collider::cuboid(100.0, 0.1, 100.0), Ground))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
}

fn spawn_ball(mut commands: Commands, time: Res<Time>, mut config: ResMut<BallSpawnConfig>) {
    config.timer.tick(time.delta());
    if config.timer.finished() {
        commands
            .spawn(RigidBody::Dynamic)
            .insert(Ball)
            .insert(BallLifetime {
                timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
            })
            .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
            .insert(Collider::ball(0.5))
            .insert(AdditionalMassProperties::Mass(1000.2))
            .insert(ColliderMassProperties::Density(1000.0))
            .insert(Restitution::coefficient(0.9))
            .insert(TransformBundle::from(Transform::from_xyz(-4.0, 1.0, 0.0)))
            .insert(Friction {
                coefficient: 3.00,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(Velocity {
                linvel: Vec3::new(30.0, 1.0, 0.0),
                angvel: Vec3::new(0.2, 0.4, 0.8),
            });
    }
}

fn despawn_ball(
    mut commands: Commands,
    mut q: Query<(Entity, &mut BallLifetime)>,
    time: Res<Time>,
) {
    for (entity, mut fuse_timer) in q.iter_mut() {
        // timers gotta be ticked, to work
        fuse_timer.timer.tick(time.delta());

        // if it finished, despawn the bomb
        if fuse_timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// we want to apply the same translation to the player if he is grounded on the cube
fn move_cubes(
    time: Res<Time>,
    mut cube_q: Query<(Entity, &mut Transform, &mut Cubeovator), With<RigidBody>>,
) //,
// mut player_q: Query<(&Grounded, &mut Transform), With<Player>>)
{
    // let  (player_grounded, mut player_transform) = player_q.single_mut();

    cube_q
        .iter_mut()
        .for_each(|(_entity, mut transform, mut cubeovator)| {
            let oscillator = (time.elapsed_seconds() % (TAU as f32)).sin();
            cubeovator.oscillator = oscillator / 6.0;

            transform.translation.y += oscillator / 6.0;
            // if (player_grounded.count == 1) && (player_grounded.entities[0] == entity) {
            //     player_transform.translation.y += oscillator / 6.0;
            // }
            // println!("Sine seconds: {}", oscillator);
        });
}

#[derive(Resource)]
struct BallSpawnConfig {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

fn setup_ball_spawning(mut commands: Commands) {
    commands.insert_resource(BallSpawnConfig {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
    })
}

#[derive(Component)]
struct BallLifetime {
    /// track when the bomb should explode (non-repeating timer)
    timer: Timer,
}

#[derive(Component)]
pub struct Ball;
