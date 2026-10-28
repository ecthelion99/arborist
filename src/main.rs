use std::collections::VecDeque;

use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    light::{NotShadowCaster, NotShadowReceiver},
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
        Transform::from_xyz(5.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    let floor_plane = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(5.0)));
    let white_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..Default::default()
    });

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
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));
}

enum TreeVisualiserType {
    Limb,
    DebugNode,
}

fn build_tree_visualisation_array(
    tree: &TreeGraph<TreePayload>,
) -> Vec<(TreeVisualiserType, Transform)> {
    let mut vis_array = Vec::new();
    let root_translation = tree.get_root_node().payload.translation;
    let mut children_stack: Vec<(Vec3, &Vec<GraphNodeId>)> =
        vec![(root_translation, &tree.get_root_node().children)];

    vis_array.push((
        TreeVisualiserType::DebugNode,
        Transform::from_translation(root_translation),
    ));

    while let Some((parent_translation, node_ids)) = children_stack.pop() {
        for node_id in node_ids {
            let node = tree.get_node_by_id(*node_id).unwrap();
            let child_translation = parent_translation + node.payload.translation;
            vis_array.push((
                TreeVisualiserType::DebugNode,
                Transform::from_translation(child_translation),
            ));
            children_stack.push((child_translation, &node.children));
        }
    }

    vis_array
}

fn spawn_tree(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut tree: TreeGraph<TreePayload> = TreeGraph::new(TreePayload::new(Vec3::ZERO));
    let node1_id = tree
        .insert_node(tree.root_id, TreePayload::new(Vec3::new(0.0, 1.0, 0.0)))
        .unwrap();
    let node2_id = tree
        .insert_node(node1_id, TreePayload::new(Vec3::new(0.0, 1.0, 0.0)))
        .unwrap();
    tree.insert_node(node1_id, TreePayload::new(Vec3::new(0.0, 0.5, 0.5)));
    tree.insert_node(node2_id, TreePayload::new(Vec3::new(0.0, 0.5, -0.5)));

    let cylinder_handle = meshes.add(Cylinder::new(0.025, 1.0));
    let sphere_handle = meshes.add(Sphere::new(0.05));
    let brown_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(150.0 / 255.0, 110.0 / 255.0, 51.0 / 255.0),
        ..Default::default()
    });
    let turquoise_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgba(145.0 / 255.0, 1.0, 250.0 / 255.0, 0.50),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    let visualisation_array = build_tree_visualisation_array(&tree);

    commands
        .spawn((
            TreeComponent { tree },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::Inherited,
        ))
        .with_children(|parent| {
            for (vis_type, transform) in visualisation_array {
                match vis_type {
                    TreeVisualiserType::DebugNode => {
                        parent.spawn((
                            Mesh3d(sphere_handle.clone()),
                            MeshMaterial3d(turquoise_material_handle.clone()),
                            NotShadowCaster,
                            NotShadowReceiver,
                            transform,
                        ));
                    }
                    TreeVisualiserType::Limb => {
                        parent.spawn((
                            Mesh3d(cylinder_handle.clone()),
                            MeshMaterial3d(brown_material_handle.clone()),
                            transform,
                        ));
                    }
                }
            }
        });
}

fn spawn_cylinder(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cylinder_handle = meshes.add(Cylinder::new(0.05, 1.0));
    let brown_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(150.0 / 255.0, 110.0 / 255.0, 51.0 / 255.0),
        ..Default::default()
    });
    commands.spawn((
        Mesh3d(cylinder_handle.clone()),
        MeshMaterial3d(brown_material_handle.clone()),
        Transform::from_scale(Vec3::new(1.0, 0.5, 1.0)).with_translation(0.25 * Vec3::Y),
    ));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FreeCameraPlugin))
        .add_systems(
            Startup,
            (spawn_camera, spawn_plane, spawn_light, spawn_tree),
        )
        .run();
}
