use bevy::color::Color;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::view::RenderLayers;

// Public handle to the offscreen texture so you can use it in your UI.
#[derive(Resource, Clone)]
pub struct AxesGizmoTexture(pub Handle<Image>);

/// Synchronize AxesGizmoCamera with AxesGizmoSyncCamera
#[derive(Component)]
pub struct AxesGizmoSyncCamera;

/// Plugin for the axes gizmo
#[derive(Resource, Clone)]
pub struct AxesGizmoPlugin {
    pub colors: [Color; 3],
    pub length: f32,
    pub width: f32,
    pub rendering_layer: usize,
}

//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////
//////////////////// Don't look down. ////////////////////

impl Plugin for AxesGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AxesGizmoTexture(Handle::default()))
            .insert_resource(self.clone())
            .add_systems(Startup, setup)
            .add_systems(Update, sync);
    }
}

impl Default for AxesGizmoPlugin {
    fn default() -> Self {
        Self {
            colors: [Color::linear_rgb(1., 0., 0.), Color::linear_rgb(0., 1., 0.), Color::linear_rgb(0., 0., 1.)],
            length: 99.,
            width: 2.,
            rendering_layer: 13,
        }
    }
}

#[derive(Component)]
struct AxesGizmoCamera;

// Setup the axis mini-scene
fn setup(
    mut commands: Commands,
    plugin_config: Res<AxesGizmoPlugin>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<bevy::render::mesh::Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_axis = meshes.add(Cuboid::new(plugin_config.length, plugin_config.width, plugin_config.width));

    // X AXIS
    let mut mat_x = StandardMaterial::from_color(plugin_config.colors[0]);
    mat_x.unlit = true;
    commands.spawn((
        Mesh3d(mesh_axis.clone()),
        MeshMaterial3d(mats.add(mat_x)),
        Transform::from_translation(Vec3::X * (plugin_config.length * 0.5)),
        RenderLayers::layer(plugin_config.rendering_layer),
    ));

    // Y AXIS
    let mut mat_y = StandardMaterial::from_color(plugin_config.colors[1]);
    mat_y.unlit = true;
    commands.spawn((
        Mesh3d(mesh_axis.clone()),
        MeshMaterial3d(mats.add(mat_y)),
        Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)).with_translation(Vec3::Y * (plugin_config.length * 0.5)),
        RenderLayers::layer(plugin_config.rendering_layer),
    ));

    // Z AXIS
    let mut mat_z = StandardMaterial::from_color(plugin_config.colors[2]);
    mat_z.unlit = true;
    commands.spawn((
        Mesh3d(mesh_axis.clone()),
        MeshMaterial3d(mats.add(mat_z)),
        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_translation(Vec3::Z * (plugin_config.length * 0.5)),
        RenderLayers::layer(plugin_config.rendering_layer),
    ));

    // Create the texture
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d {
                width: 256,
                height: 256,
                ..default()
            },
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(image.texture_descriptor.size);
    let handle = images.add(image);
    commands.insert_resource(AxesGizmoTexture(handle.clone()));

    // Spawn the camera
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection::default_3d()),
        Camera {
            target: handle.into(),
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        RenderLayers::layer(plugin_config.rendering_layer),
        AxesGizmoCamera,
    ));
}

// Synchronize AxesGizmoCamera with AxesGizmoSyncCamera
fn sync(mut axis_cam_q: Query<&mut Transform, With<AxesGizmoCamera>>, main_cameras_q: Query<&GlobalTransform, With<AxesGizmoSyncCamera>>) {
    if let Ok(mut axis_t) = axis_cam_q.single_mut() {
        if let Ok(main_gt) = main_cameras_q.single() {
            let rot = main_gt.rotation();

            let eye = -(rot * -Vec3::Z) * 100.;
            *axis_t = Transform::from_translation(eye);

            let up = rot * Vec3::Y;
            axis_t.look_at(Vec3::ZERO, up);
        }
    }
}
