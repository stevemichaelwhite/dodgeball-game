use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_directional_light, spawn_floor));
    }
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };
    commands.spawn(light);
}

fn spawn_directional_light(mut commands: Commands) {
    let light = DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 1_500.,
            ..default()
        },
        ..default()
    };
    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let debug_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(images.add(uv_debug_texture())),
    //     ..default()
    // });

    let floor = PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0).subdivisions(10)),
        // mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(StandardMaterial {
            base_color: Srgba::hex("#21ad1a").unwrap().into(),
            metallic: (-1 + 2) as f32 / 4.0,
            perceptual_roughness: (-4 + 5) as f32 / 10.0,
            ..Default::default()
        }),
        ..default()
    };
    commands.spawn(floor);
}
