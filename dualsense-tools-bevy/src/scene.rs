use bevy::camera::primitives::Aabb;
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;
use bevy_third_person_camera::*;
use dualsense_tools::{Dualsense, Tilt, TiltEstimator};
use std::sync::{Arc, Mutex};

#[derive(Resource)]
struct GamepadRes(Arc<Mutex<Dualsense>>);

#[derive(Resource)]
struct TiltEstimatorRes<const BUFSIZE: usize>(TiltEstimator<BUFSIZE>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TiltAlg {
    RawAccel,
    RawGyro,
    AccelQuaternion,
    GyroQuaternion,
    Integrated,
}

#[derive(Component)]
struct GamepadBound {
    tilt_alg: TiltAlg,
}

#[derive(Component)]
struct Diagnostics;

#[derive(Component)]
struct ShowAxes;

pub fn scene<const BUFSIZE: usize>(
    dualsense: Arc<Mutex<Dualsense>>,
    tilt_estimator: TiltEstimator<BUFSIZE>,
) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ThirdPersonCameraPlugin)
        .insert_resource(GamepadRes(dualsense))
        .insert_resource(TiltEstimatorRes(tilt_estimator))
        .add_systems(Startup, setup)
        .add_systems(Update, update::<BUFSIZE>)
        .add_systems(Update, draw_axes)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_settings = ThirdPersonCamera {
        mouse_orbit_button_enabled: true,
        mouse_orbit_button: MouseButton::Right,
        ..Default::default()
    };

    commands.spawn((
        Camera3d::default(),
        camera_settings,
        Transform::from_xyz(0., 0., 6.),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1., 0., 0.),
            metallic: 0.8,
            ..default()
        })),
        ThirdPersonCameraTarget,
        Transform::from_xyz(0.0, 1., 0.0),
        GamepadBound {
            tilt_alg: TiltAlg::AccelQuaternion,
        },
        ShowAxes,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1., 0., 0.),
            metallic: 0.8,
            ..default()
        })),
        ThirdPersonCameraTarget,
        Transform::from_xyz(2., 1., 0.),
        GamepadBound {
            tilt_alg: TiltAlg::GyroQuaternion,
        },
        ShowAxes,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1., 0., 0.),
            metallic: 0.8,
            ..default()
        })),
        ThirdPersonCameraTarget,
        Transform::from_xyz(-2., 1., 0.),
        GamepadBound {
            tilt_alg: TiltAlg::Integrated,
        },
        ShowAxes,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0., 1., 0.),
            metallic: 0.8,
            ..default()
        })),
        ThirdPersonCameraTarget,
        Transform::from_xyz(-1., -1., 0.),
        GamepadBound {
            tilt_alg: TiltAlg::RawAccel,
        },
        ShowAxes,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0., 1., 0.),
            metallic: 0.8,
            ..default()
        })),
        ThirdPersonCameraTarget,
        Transform::from_xyz(1., -1., 0.),
        GamepadBound {
            tilt_alg: TiltAlg::RawGyro,
        },
        ShowAxes,
    ));

    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));

    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
        Diagnostics,
    ));
}

fn draw_axes(mut gizmos: Gizmos, query: Query<(&Transform, &Aabb), With<ShowAxes>>) {
    for (&transform, &aabb) in &query {
        let length = aabb.half_extents.length();
        gizmos.axes(transform, length);
    }
}

fn update<const S: usize>(
    gamepad: Res<GamepadRes>,
    tilt_estimator: ResMut<TiltEstimatorRes<S>>,
    cubes: Query<(&mut Transform, &GamepadBound)>,
    time: Res<Time>,
) {
    let state = gamepad.0.lock().unwrap().read().unwrap();
    let estimates =
        tilt_estimator
            .into_inner()
            .0
            .next_estimate(&state.accel, &state.gyro, &time.delta());

    for (mut transform, config) in cubes {
        let new_tilt = match config.tilt_alg {
            TiltAlg::AccelQuaternion => estimates.accel,
            TiltAlg::GyroQuaternion => Tilt::default(),
            TiltAlg::RawAccel => Tilt::default(),
            TiltAlg::RawGyro => estimates.integrated_gyro,
            TiltAlg::Integrated => estimates.fused,
        };

        transform.rotation = Quat::from_array(new_tilt.quat.to_array());
    }
}
