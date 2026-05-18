use bevy::{
    input::gamepad::{GamepadConnection, GamepadConnectionEvent},
    log,
    prelude::*,
};
use dualsense_tools::*;
use hidapi::HidApi;
use std::sync::{Arc, Mutex};

/// A plugin that integrate the tilt estimation algorithm implemented
/// in the dualsense-tools crate as a bevy plugin.
#[derive(Default, Debug)]
pub struct DualsenseTiltPlugin<const SAMPLES: usize>;

impl<const SAMPLES: usize> Plugin for DualsenseTiltPlugin<SAMPLES> {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(DualsenseTilt::default())
            .insert_resource(DualsenseResource::default())
            .insert_resource(TiltEstimatorResource::new(
                TiltEstimatorConfig::<SAMPLES>::default(),
            ))
            .add_systems(Update, handle_connection)
            .add_systems(
                Update,
                update_tilt_tilt_system::<SAMPLES>.pipe(handle_results),
            );
    }
}

/// A resource that will be updated by the plugin with the latest
/// values produced by the tilt estimator
#[derive(Resource, Default, Clone, Copy, Debug)]
pub struct DualsenseTilt(TiltEstimates);

impl DualsenseTilt {
    pub fn estimates(&self) -> TiltEstimates {
        self.0
    }
}

#[derive(Resource, Clone, Debug, Default)]
struct DualsenseResource {
    dualsense: Option<Arc<Mutex<Dualsense>>>,
}

#[derive(Resource, Clone, Debug)]
struct TiltEstimatorResource<const SAMPLES: usize> {
    tilt_estimator: TiltEstimator<SAMPLES>,
}

impl<const SAMPLES: usize> TiltEstimatorResource<SAMPLES> {
    fn new(config: TiltEstimatorConfig<SAMPLES>) -> TiltEstimatorResource<SAMPLES> {
        TiltEstimatorResource {
            tilt_estimator: TiltEstimator::new(config),
        }
    }
}

fn update_tilt_tilt_system<const SAMPLES: usize>(
    controller_res: Res<DualsenseResource>,
    tilt: ResMut<DualsenseTilt>,
    estimator: ResMut<TiltEstimatorResource<SAMPLES>>,
    time: Res<Time>,
) -> Result<(), BevyError> {
    tilt.into_inner().0 = if let Some(controller) = &controller_res.into_inner().dualsense {
        let state = controller.lock().unwrap().read()?;
        estimator.into_inner().tilt_estimator.next_estimate(
            &state.accel,
            &state.gyro,
            &time.delta(),
        )
    } else {
        TiltEstimates::default()
    };

    Ok(())
}

fn handle_connection(
    mut connection_events: MessageReader<GamepadConnectionEvent>,
    controller_res: ResMut<DualsenseResource>,
) -> Result<(), BevyError> {
    for ev in connection_events.read() {
        match &ev.connection {
            GamepadConnection::Connected {
                name: _,
                vendor_id,
                product_id,
            } => {
                if *vendor_id == Some(VENDOR_ID) && *product_id == Some(PRODUCT_ID) {
                    log::info!("Connecting Dualsense Controller");
                    let mut hidapi = HidApi::new()?;
                    let dualsense = Dualsense::new(&mut hidapi)?;
                    controller_res.into_inner().dualsense = Some(Arc::new(Mutex::new(dualsense)));
                    break;
                }
            }

            GamepadConnection::Disconnected => (),
        }
    }

    Ok(())
}

fn handle_results(r: In<Result<(), BevyError>>, controller_res: ResMut<DualsenseResource>) {
    match r.0 {
        Ok(()) => (),
        Err(err) => {
            bevy::log::error!(
                "Error in the dualsense-tilt plugin, disconnecting controller: {}",
                err
            );
            controller_res.into_inner().dualsense = None;
        }
    }
}
