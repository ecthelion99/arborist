use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    prelude::*,
};
use glam::Vec3;

mod tree;
use tree::*;

#[derive(Component)]
struct TreeComponent {
    tree: TreeGraph<TreePayload>,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.0, 0.5, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera {
            sensitivity: 0.2,
            friction: 25.0,
            walk_speed: 3.0,
            run_speed: 9.0,
            ..default()
        },
    ));
}

fn spawn_plane(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let floor_plane = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(2.0)));
    let white_material = materials.add(Color::WHITE);

    commands.spawn((
        Mesh3d(floor_plane.clone()),
        MeshMaterial3d(white_material.clone()),
    ));
}

fn spawn_light(mut commands: Commands) {
    // Main light
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));
}

fn spawn_tree(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut tree: TreeGraph<TreePayload> = TreeGraph::new(TreePayload::new(Vec3::ZERO));
    let node1_id = tree.insert_node(tree.root_id, TreePayload::new(Vec3::new(0.0, 1.0, 0.0)));
    tree.insert_node(node1_id, TreePaload::new(Vec))
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FreeCameraPlugin))
        .add_systems(Startup, (spawn_camera, spawn_plane, spawn_light))
        .run();
}
