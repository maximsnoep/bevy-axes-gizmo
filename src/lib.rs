//! A small view of the X, Y and Z axes that turns with a camera, rendered to a texture (to show in the UI). Its colors
//! can change at any time (`AxesGizmoColors`).

use bevy::camera::RenderTarget;
use bevy::camera::visibility::RenderLayers;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use std::f32::consts::FRAC_PI_2;

/// The axes gizmo: its initial colors, the length and width of the axes, and the render layer of its scene.
#[derive(Clone)]
pub struct AxesGizmoPlugin {
    pub colors: [Color; 3],
    pub length: f32,
    pub width: f32,
    pub rendering_layer: usize,
}

impl Default for AxesGizmoPlugin {
    fn default() -> Self {
        Self {
            colors: [
                Color::linear_rgb(1., 0., 0.),
                Color::linear_rgb(0., 1., 0.),
                Color::linear_rgb(0., 0., 1.),
            ],
            length: 99.,
            width: 2.,
            rendering_layer: 13,
        }
    }
}

/// The colors of the X, Y and Z axes (the gizmo follows when they change).
#[derive(Resource, Clone, PartialEq)]
pub struct AxesGizmoColors(pub [Color; 3]);

/// The texture the gizmo is rendered to.
#[derive(Resource, Clone)]
pub struct AxesGizmoTexture(pub Handle<Image>);

/// The camera whose rotation the gizmo follows.
#[derive(Component)]
pub struct AxesGizmoSyncCamera;

#[derive(Component)]
struct AxesGizmoCamera;

// An axis of the gizmo (0, 1, 2: X, Y, Z).
#[derive(Component)]
struct Axis(usize);

#[derive(Resource, Clone)]
struct Config {
    length: f32,
    width: f32,
    layer: usize,
}

impl Plugin for AxesGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AxesGizmoColors(self.colors))
            .insert_resource(AxesGizmoTexture(Handle::default()))
            .insert_resource(Config {
                length: self.length,
                width: self.width,
                layer: self.rendering_layer,
            })
            .add_systems(Startup, setup)
            .add_systems(Update, (follow_camera, recolor));
    }
}

// The axes (boxes along X, turned onto their axis), the texture, and the camera of the gizmo.
fn setup(
    mut commands: Commands,
    config: Res<Config>,
    colors: Res<AxesGizmoColors>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layer = RenderLayers::layer(config.layer);
    let mesh = meshes.add(Cuboid::new(config.length, config.width, config.width));
    let turns = [
        Quat::IDENTITY,
        Quat::from_rotation_z(FRAC_PI_2),
        Quat::from_rotation_y(FRAC_PI_2),
    ];
    for (axis, (turn, direction)) in turns.into_iter().zip([Vec3::X, Vec3::Y, Vec3::Z]).enumerate() {
        let material = materials.add(StandardMaterial {
            base_color: colors.0[axis],
            unlit: true,
            ..default()
        });
        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_rotation(turn).with_translation(direction * config.length * 0.5),
            layer.clone(),
            Axis(axis),
        ));
    }

    let size = Extent3d {
        width: 256,
        height: 256,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let handle = images.add(image);
    commands.insert_resource(AxesGizmoTexture(handle.clone()));

    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection::default_3d()),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        Tonemapping::None,
        DebandDither::Disabled,
        RenderTarget::Image(handle.into()),
        layer,
        AxesGizmoCamera,
    ));
}

// The gizmo's camera looks at the axes as the synced camera looks at the scene.
fn follow_camera(
    mut gizmo_camera: Query<&mut Transform, With<AxesGizmoCamera>>,
    synced: Query<&GlobalTransform, With<AxesGizmoSyncCamera>>,
) {
    if let Ok(mut transform) = gizmo_camera.single_mut()
        && let Ok(synced) = synced.single()
    {
        let rotation = synced.rotation();
        *transform = Transform::from_translation(rotation * Vec3::Z * 100.).looking_at(Vec3::ZERO, rotation * Vec3::Y);
    }
}

// The axes in their current colors.
fn recolor(
    colors: Res<AxesGizmoColors>,
    axes: Query<(&Axis, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !colors.is_changed() {
        return;
    }
    for (axis, material) in &axes {
        if let Some(mut material) = materials.get_mut(&material.0) {
            material.base_color = colors.0[axis.0];
        }
    }
}
