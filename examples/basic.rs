use bevy::prelude::*;
use bevy_axes_gizmo::{AxesGizmoPlugin, AxesGizmoSyncCamera, AxesGizmoTexture};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AxesGizmoPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(PostStartup, setup_ui)
        .add_systems(Update, orbit_camera)
        .run();
}

fn orbit_camera(time: Res<Time>, mut query: Query<&mut Transform, With<AxesGizmoSyncCamera>>) {
    let t = time.elapsed_secs() * 0.5;
    for mut transform in &mut query {
        let x = t.sin() * 5.0;
        let z = t.cos() * 5.0;
        *transform = Transform::from_xyz(x, 2.5, z).looking_at(Vec3::ZERO, Vec3::Y);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        AxesGizmoSyncCamera,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Transform::default(),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// PostStartup ensures the plugin's Startup commands have been flushed,
// so AxesGizmoTexture holds the real render-target handle by the time we read it.
fn setup_ui(mut commands: Commands, axes_gizmo_image: Res<AxesGizmoTexture>) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            bottom: Val::Px(0.),
            width: Val::Px(128.),
            height: Val::Px(128.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageNode {
                image: axes_gizmo_image.0.clone(),
                ..default()
            });
        });
}
