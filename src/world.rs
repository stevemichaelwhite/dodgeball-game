use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_light,
                spawn_directional_light,
                spawn_floor,
                spawn_objects,
            ),
        );
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
            mesh: meshes.add(Plane3d::default().mesh().size(ground_size, ground_size).subdivisions(10)),
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
        Collider::cuboid(ground_size/2.0, ground_height, ground_size/2.0),
        // RigidBody::Fixed,
    );
    commands.spawn(floor);
    
}

#[derive(Component)]
pub struct Cube;


fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    let mut create_cube =
        |hdim_xyz: (f32, f32, f32), color_hex: String, xyz: (f32, f32, f32), name: String| -> (PbrBundle, Name, Collider, RigidBody, Cube) {
            (
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(hdim_xyz.0,hdim_xyz.1,hdim_xyz.2)),
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
                Collider::cuboid(hdim_xyz.0/2.0, hdim_xyz.1/2.0, hdim_xyz.2/2.0),
                RigidBody::KinematicPositionBased,
                Cube
            )
        };
    commands.spawn(create_cube(
        (2.0, 2.0, 2.0),
        "#1a1fad".to_string(),
        (1.7, 1.0, 0.0),
        "BlueCube".to_string(),
    ));

    commands.spawn(create_cube(
        (2.0, 2.0, 2.0),
        "#ad1a30".to_string(),
        (-3.3, 1.0, 3.5),
        "RedCube".to_string(),
    ));

}

