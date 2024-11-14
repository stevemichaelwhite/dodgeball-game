use bevy::prelude::*;

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
    let floor = (
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0).subdivisions(10)),
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
    );
    commands.spawn(floor);
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    let mut create_cube =
        |depth: f32, color_hex: String, xyz: (f32, f32, f32), name: String| -> (PbrBundle, Name) {
            (
                PbrBundle {
                    mesh: meshes.add(Extrusion::new(Rectangle::default(), depth)),
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
            )
        };
    commands.spawn(create_cube(
        4.0,
        "#1a1fad".to_string(),
        (1.7, 0.5, 0.0),
        "BlueCube".to_string(),
    ));

    commands.spawn(create_cube(
        2.0,
        "#ad1a30".to_string(),
        (-3.3, 0.5, 3.5),
        "RedCube".to_string(),
    ));

}
