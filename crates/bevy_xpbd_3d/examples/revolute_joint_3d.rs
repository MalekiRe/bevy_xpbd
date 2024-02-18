use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};
use examples_common_3d::XpbdExamplePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, XpbdExamplePlugin))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Msaa::Sample4)
        .insert_resource(SubstepCount(50))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_mesh = meshes.add(Cuboid::default());
    let cube_material = materials.add(Color::rgb(0.8, 0.7, 0.6));

    // Kinematic rotating "anchor" object
    let anchor = commands
        .spawn((
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: cube_material.clone(),
                ..default()
            },
            RigidBody::Kinematic,
            AngularVelocity(Vector::Z * 1.5),
        ))
        .id();

    // Dynamic object rotating around anchor
    let object = commands
        .spawn((
            PbrBundle {
                mesh: cube_mesh,
                material: cube_material,
                transform: Transform::from_xyz(0.0, -2.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::cuboid(1.0, 1.0, 1.0), 1.0),
        ))
        .id();

    // Connect anchor and dynamic object
    commands.spawn(
        RevoluteJoint::new(anchor, object)
            .with_local_anchor_2(Vector::Y * 2.0)
            .with_angle_limits(-1.0, 1.0),
    );

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(Vec3::new(-1.0, -2.5, -1.5), Vec3::Y),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::Z * 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
