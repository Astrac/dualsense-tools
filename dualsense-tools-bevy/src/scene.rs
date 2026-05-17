use bevy::camera::primitives::Aabb;
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;
use bevy_third_person_camera::*;

use crate::plugin::{DualsenseTiltPlugin, DualsenseTilt};

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

pub fn scene<const BUFSIZE: usize>() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(DualsenseTiltPlugin::<BUFSIZE>)
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
        Transform::from_xyz(-2., 1., 0.0),
        GamepadBound {
            tilt_alg: TiltAlg::AccelInstant,
        },
        ShowAxes,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0., 0., 1.),
            metallic: 0.8,
            ..default()
        })),
        ThirdPersonCameraTarget,
        Transform::from_xyz(0., 1., 0.),
        GamepadBound {
            tilt_alg: TiltAlg::GyroInstant,
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
        Transform::from_xyz(2., 1., 0.),
        GamepadBound {
            tilt_alg: TiltAlg::AccelAverage,
        },
        ShowAxes,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0., 1., 1.),
            metallic: 0.8,
            ..default()
        })),
        ThirdPersonCameraTarget,
        Transform::from_xyz(0., -1., 0.),
        GamepadBound {
            tilt_alg: TiltAlg::AccelCorrectedGyro,
        },
        ShowAxes,
    ));

    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));
}

fn draw_axes(mut gizmos: Gizmos, query: Query<(&Transform, &Aabb), With<ShowAxes>>) {
    for (&transform, &aabb) in &query {
        let length = aabb.half_extents.length();
        gizmos.axes(transform, length);
    }
}

fn update<const S: usize>(tilt: Res<DualsenseTilt>, cubes: Query<(&mut Transform, &GamepadBound)>) {
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
