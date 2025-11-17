[![Crates.io](https://img.shields.io/crates/v/bevy_axes_gizmo)](https://crates.io/crates/bevy_axes_gizmo)

# Bevy Axes Gizmo

![bevy-axes-gizmo-demo](https://github.com/user-attachments/assets/d7fb06cd-fdfa-404c-a50b-b581fa55dd8d)


## Summary

Bevy Axes Gizmo is a plugin for the [Bevy](https://bevyengine.org) engine that provides a simple gizmo for visualizing the three coordinate axes of the scene, synchronized with the orientation of the main camera. 

## How to use

To render your axes gizmo, add the crate to your project with `cargo add bevy-axes-gizmo` and follow these three steps:

1. Add the `AxesGizmoPlugin` to your app.
```rust
App::new()
  .add_plugins(AxesGizmoPlugin::default())
```

2. Add the `AxesGizmoSyncCamera` marker type to your main (moving) camera. This will synchronize the orientation of the axes with the orientation of the camera.
```rust
commands
  .spawn((
    Camera3d::default(),
    Camera::default(),
    AxesGizmoSyncCamera,
  ))
```

3. Then, the bevy resource `AxesGizmoTexture` has a handle to the image of the axes gizmo. You can use this image in various ways. Below, you can find an example of how to render the image given an absolute position.
```rust
fn render_axes_gizmo(
  axes_gizmo_image: Res<AxesGizmoTexture>
) {
  commands
    .spawn((
      Node {
        position_type: PositionType::Absolute,
        left: Val::Px(0.),
        bottom: Val::Px(0.),
        width: Val::Px(128.),
        height: Val::Px(128.),
        ..default()
      },
      BackgroundColor(Color::NONE),
    ))
    .with_children(|parent| {
      parent.spawn(ImageNode {
        image: axes_gizmo_image.0.clone(),
        ..default()
      });
    });
}
```

## Customization

While instantiating the plugin you can customize the axes by supplying values for its colors, length, width, and rendering layer:
```rust
pub struct AxesGizmoPlugin {
    pub colors: [Color; 3],
    pub length: f32,
    pub width: f32,
    pub rendering_layer: usize,
}
```

You can do the following to change the colors of the X-, Y-, and Z-axis to red, yellow, and blue:
```rust
App::new()
  .add_plugins(AxesGizmoPlugin {
    colors: [
      Color::srgb(0.95, 0.5, 0.5), 
      Color::srgb(0.95, 0.95, 0.7),
      Color::srgb(0.5, 0.5, 0.95)
    ],
    ..default()
  })
```



## Version Compatibility

| bevy | bevy_axes_gizmo |
|------|-----------------|
| 0.16.2 | 0.1.x |

