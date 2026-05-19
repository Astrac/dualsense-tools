/// This example shows how to use the tilt estimator plugin to manipulate
/// objects on a scene; it also works as a showcase of the estimate values
/// produced by the estimator.
use bevy::camera::ScalingMode;
use bevy::camera::primitives::Aabb;
use bevy::prelude::*;
use dualsense_tools_bevy::{DualsenseTilt, DualsenseTiltPlugin};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TiltAlg {
    AccelAverage,
    AccelInstant,
    AccelCorrectedGyro,
    GyroInstant,
}

#[derive(Component)]
struct GamepadBound {
    tilt_alg: TiltAlg,
}

#[derive(Component)]
struct ShowAxes;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "examples/assets".to_owned(),
            ..default()
        }))
        .add_plugins(DualsenseTiltPlugin::<10>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_systems(Update, draw_axes)
        .add_systems(Update, attach_labels)
        .run();
}

fn spawn_tilt_visualizer(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    color: Color,
    tilt_alg: TiltAlg,
    transform: Transform,
    text: impl Into<String>,
    font: TextFont,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            metallic: 0.8,
            ..default()
        })),
        transform,
        GamepadBound { tilt_alg },
        ShowAxes,
        Node {
            top: Val::Px(0.),
            left: Val::Px(0.),
            width: Val::Auto,
            height: Val::Auto,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Text::new(text),
        TextColor(color),
        TextBackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        TextLayout::new_with_justify(Justify::Center),
        font,
    ));
}

fn attach_labels(
    cubes: Query<(&Transform, &mut Node)>,
    camera: Single<(Entity, &Camera)>,
    transform_helper: TransformHelper,
) {
    let Ok(camera_transform) = transform_helper.compute_global_transform(camera.0) else {
        warn!("Failed computing global transform for camera Entity");
        return;
    };

    for (transform, mut node) in cubes {
        let translation = transform.translation;
        let global_transform = camera
            .1
            .world_to_viewport(
                &camera_transform,
                translation
                    .with_y(translation.y + 1.0)
                    .with_x(translation.x - 0.4),
            )
            .unwrap();
        node.top = Val::Px(global_transform.y);
        node.left = Val::Px(global_transform.x);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("FantasqueSansMono-Bold.otf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 13.0,
        ..default()
    };

    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 4.5,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 0., 3.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    spawn_tilt_visualizer(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(1., 0., 0.),
        TiltAlg::AccelInstant,
        Transform::from_xyz(-2., 0.8, 0.),
        "Accelerometer\nInstant Reading",
        text_font.clone(),
    );

    spawn_tilt_visualizer(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0., 0., 1.),
        TiltAlg::GyroInstant,
        Transform::from_xyz(0., 0.8, 0.),
        "Gyroscope\nInstant Reading",
        text_font.clone(),
    );

    spawn_tilt_visualizer(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0., 1., 0.),
        TiltAlg::AccelAverage,
        Transform::from_xyz(2., 0.8, 0.),
        "Accelerometer\nAverage",
        text_font.clone(),
    );

    spawn_tilt_visualizer(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0., 1., 1.),
        TiltAlg::AccelCorrectedGyro,
        Transform::from_xyz(0., -1.2, 0.),
        "Accelerometer-corrected\ngyroscope",
        text_font.clone(),
    );

    commands.spawn((PointLight::default(), Transform::from_xyz(0.0, 8.0, 7.0)));
}

fn draw_axes(mut gizmos: Gizmos, query: Query<(&Transform, &Aabb), With<ShowAxes>>) {
    for (&transform, &aabb) in &query {
        let length = aabb.half_extents.length();
        gizmos.axes(transform, length);
    }
}

fn update(tilt: Res<DualsenseTilt>, cubes: Query<(&mut Transform, &GamepadBound)>) {
    for (mut transform, config) in cubes {
        let estimates = tilt.estimates();
        let new_tilt = match config.tilt_alg {
            TiltAlg::AccelAverage => estimates.accel_avg,
            TiltAlg::AccelInstant => estimates.accel_instant,
            TiltAlg::GyroInstant => estimates.gyro_instant,
            TiltAlg::AccelCorrectedGyro => estimates.accel_corrected_gyro,
        };

        transform.rotation = Quat::from_array(new_tilt.quat.to_array());
    }
}
