# Dualsense tilt plugin

This plugin implements systems to manage the tilt-estimation functionality exposed by `dualsense-tools` within bevy; enabling this plugin as follows:

```
app.add_plugins(DualsenseTiltPlugin::<BUFSIZE>)
```

The `BUFSIZE` parameter decides how big the buffer of accelerometer readings will be, deciding how many samples of will be used in the smoothing of tilt estimate.

The plugin will insert a `DualsenseTilt` resource that is updated by using the tilt-estimation algorithm implement in `dualsense-tools`; this can be used in a system as in the following example:

```
fn update(tilt: Res<DualsenseTilt>, controlled: Query<(&mut Transform, &ControlledMesh)>) {
    for (mut transform, _) in controlled {
        transform.rotation = Quat::from_array(tilt.estimates().accel_corrected_gyro);
    }
}
```

For a more detailed example refer to [visualizer](./examples/visualizer.rs), which you can run as usual:

```
cargo run -p dualsense-tools-bevy --example visualizer
```
