use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 981.0))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // 2Dカメラの生成
    commands.spawn(Camera2d);

    // 1. 床
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(600.0, 40.0),
        Restitution::new(1.0),
        Friction::new(0.0),
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));

    // 2. 落ちてくるボール
    commands.spawn((
        RigidBody::Dynamic,
        Collider::circle(20.0),
        Restitution::new(1.0),
        Friction::new(0.0),
        Transform::from_xyz(0.0, 200.0, 0.0),
    ));
}
