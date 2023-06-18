# Bevy XPBD

**Bevy XPBD** is a 2D and 3D physics engine based on *Extended Position Based Dynamics* (XPBD) for the [Bevy game engine](https://bevyengine.org/). The *Entity Component System* (ECS) is used heavily throughout the engine to enable enhanced parallellism, configurability and familiarity, while making the engine fit better into the Bevy ecosystem.

XPBD is an improved variant of traditional *Position Based Dynamics* (PBD). It provides unconditionally stable, time step independent and physically accurate simulations that use simple constraint projection to handle things like contacts, joints, and interactions between rigid bodies, soft bodies and fluids.

The current supported version of Bevy is 0.10.1.

## Stability warning

Bevy XPBD is in early development, and it has not been released on [crates.io](https://crates.io) yet. There are several stability and performance issues, missing features, and a lack of proper documentation, so for the time being, I recommend using alternatives like [bevy_rapier](https://github.com/dimforge/bevy_rapier) for any serious projects.

That being said, I hope to release 0.1.0 in the not-so-distant future, and I plan to eventually reach feature parity with established physics engines. In the meantime, feel free to experiment with the engine, and consider opening issues or pull requests for any problems you may encounter.

## Usage example

Bevy XPBD isn't available on [crates.io](https://crates.io) yet, so you will have to specify the dependency using the Git repository:

```toml
[dependencies]
bevy_xpbd_3d = { git = "https://github.com/Jondolf/bevy_xpbd.git" }
# ...other dependencies
```

Below is a very simple example where a box with initial angular velocity falls onto a plane. This is a modified version of Bevy's [3d_scene](https://bevyengine.org/examples/3d/3d-scene/) example.

```rs
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(8.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 0.002, 8.0),
    ));
    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        },
        RigidBody::Dynamic,
        Position(Vec3::Y * 4.0),
        AngularVelocity(Vec3::new(2.5, 3.4, 1.6)),
        Collider::cuboid(1.0, 1.0, 1.0),
    ));
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 6.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
```

https://user-images.githubusercontent.com/57632562/230185604-b40441a2-48d8-4566-9b9e-be4825f4877e.mp4

## More examples

You can find lots of 2D and 3D examples in [/crates/bevy_xpbd_2d/examples](/crates/bevy_xpbd_2d/examples) and [/crates/bevy_xpbd_3d/examples](/crates/bevy_xpbd_3d/examples) respectively.

The examples support both `f32` and `f64` precisions, so the code contains some feature-dependent types like `Scalar` and `Vector`.
In actual usage these are not needed, so you can just use `f32` or `f64` types depending on the features you have chosen.

By default the examples use `f64`. To run the `f32` versions, you need to disable default features and manually choose the dimension
and precision:

```
cargo run --example cubes --no-default-features --features 3d,f32
```

## Current features

- 2D and 3D support
- Dynamic, kinematic and static rigid bodies
- Collision detection powered by [parry](https://parry.rs)
  - Collision events
  - Collision layers
  - Sensor colliders
- Basic joints
  - Revolute joint (or hinge joint), optional angle limits
  - Spherical joint, optional swing and twist angle limits
  - Prismatic joint, one free translational axis with optional limits
  - Fixed joint
- Joint damping
- Modular plugin architecture
- Support for custom constraints
- Gravity
- External forces
- Restitution
- Friction
- Substepping
- Configurable timesteps
- Sleeping
- Choose between `f32` and `f64`

## Future features

- On-demand simulation stepping
- Linear and angular velocity damping
- Locking translational and rotational axes without joints
- Joint motors
- Spatial queries
- Continuous collision detection
- Multiple colliders per body
- Debug render colliders, joints, contacts etc.
- Performance optimization (better broad phase, parallel solver...)
- Soft bodies
  - Cloth
  - Deformable solids
- Maybe fluid simulation

## Inspirations and resources

I recommend checking out [Johan Helsing's](https://github.com/johanhelsing) amazing (but incomplete) [tutorial series on XPBD in Bevy](https://johanhelsing.studio/posts/bevy-xpbd). He inspired and helped me build this engine, and the series serves as a great practical walkthrough of how to make a basic XPBD physics engine with Bevy's ECS.

To understand the algorithm better, it's also worth checking out some of the papers:

  - Müller M, Macklin M, Chentanez N, Jeschke S, Kim T. 2020. *[Detailed Rigid Body Simulation with Extended Position Based Dynamics](https://matthias-research.github.io/pages/publications/PBDBodies.pdf)*.
  - Macklin M, Müller M, Chentanez N. 2016. *[XPBD: Position-Based Simulation of Compliant Constrained Dynamics](http://mmacklin.com/xpbd.pdf)*.

## License

Bevy XPBD is free and open source. All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
