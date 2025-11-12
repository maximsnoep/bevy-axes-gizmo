# bevy-axes-gizmo
Bevy plugin for axes gizmo.

To use, add the `AxesGizmoSyncCamera` marker type to a camera. This will make the axes follow this camera.

Add the `AxesGizmoPlugin` to your app. This updates the axes  texture every frame.
```
App::new()
  .add_plugins(AxesGizmoPlugin)
```

Then, the bevy resource `AxesGizmoTexture` has a handle to the image of the axes. It can then be added to the ui. This is an example of how to do so as a child of an absolute positioned node.
```
fn add_axes_texture(
  axes_texture: Res<AxesGizmoTexture>
) {
  commands
    .spawn((
      Node {
        position_type: PositionType::Absolute,
        left: Val::Px(left),
        bottom: Val::Px(bottom),
        width: Val::Px(128.0),
        height: Val::Px(128.0),
        ..default()
      },
      BackgroundColor(Color::NONE),
    ))
    .with_children(|parent| {
    // Image node that uses the gizmo texture
      parent.spawn(ImageNode {
        image: axes_texture.0.clone(),
        ..default()
      });
    });
}
```
