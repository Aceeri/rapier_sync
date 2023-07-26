use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Startup, || {
            info!("Press `T` to toggle hierarchy parent for child colliders");
            info!("Press `P` to display discrepancies in hierarchy vs collider parents");
        })
        .add_systems(Update, (print_parents, toggle_parent))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-30.0, 30.0, 100.0)
            .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    let ground_size = 200.1;
    let ground_height = 0.1;

    commands.spawn((
        Name::new("Ground"),
        TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));

    let size = 5.0;
    commands.spawn((
        Name::new("Collider/rigid body, same entity"),
        RigidBody::Dynamic,
        //Collider::cuboid(size, size, size),
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(0., 2., 0.),
                ..default()
            },
            ..default()
        },
    ));

    let parent1 = commands
        .spawn((
            Name::new("Parent 1"),
            RigidBody::Dynamic,
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(size * 2.0, 2., 0.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let parent2 = commands
        .spawn((
            Name::new("Parent 2"),
            RigidBody::Dynamic,
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(-size * 2.0, 2., 0.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let child = commands
        .spawn((
            Name::new("Child"),
            Collider::cuboid(size, size, size),
            SpatialBundle {
                transform: Transform::default(),
                ..default()
            },
            ParentToggle {
                parent1: parent1,
                parent2: parent2,
            },
        ))
        .id();

    commands.entity(parent1).add_child(child);
}

#[derive(Component)]
pub struct ParentToggle {
    parent1: Entity,
    parent2: Entity,
}

pub fn toggle_parent(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    to_toggle: Query<(Entity, &Parent, &ParentToggle)>,
) {
    if !input.just_pressed(KeyCode::T) {
        return;
    }
    info!("toggling parents");

    for (entity, parent, toggle) in &to_toggle {
        let current = parent.get();

        let new = if current == toggle.parent1 {
            toggle.parent2
        } else {
            toggle.parent1
        };

        commands.entity(entity).set_parent(new);
    }
}

pub fn print_masses(
    input: Res<Input<KeyCode>>,
    ctx: Res<RapierContext>,
    colliders: Query<(Entity, &Parent), With<Collider>>,
    names: Query<DebugName>,
) {
    if !input.just_pressed(KeyCode::P) {
        return;
    }

    for (collider, parent) in &colliders {
        let rigidbody = ctx.collider_parent(collider);
        if rigidbody != Some(parent.get()) {
            info!("collider parent does not match hierarchy parent");
            info!("collider: {:?}", names.get(collider));
            info!(
                "found collider parent: {:?}",
                rigidbody.map(|entity| names.get(entity))
            );
            info!("found hierarchy parent: {:?}", names.get(parent.get()));
        }
    }
}
