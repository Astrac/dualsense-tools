use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use dualsense_tools::{Dualsense, TiltEstimates, TiltEstimator, TiltEstimatorConfig};

#[derive(Resource, Clone, Debug)]
pub struct DualsenseController {
    dualsense: Arc<Mutex<Dualsense>>,
}

impl DualsenseController {
    fn new(dualsense: Arc<Mutex<Dualsense>>) -> DualsenseController {
        DualsenseController { dualsense }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct TiltEstimatorResource<const SAMPLES: usize> {
    tilt_estimator: TiltEstimator<SAMPLES>,
}

impl<const SAMPLES: usize> TiltEstimatorResource<SAMPLES> {
    fn new(config: TiltEstimatorConfig<SAMPLES>) -> TiltEstimatorResource<SAMPLES> {
        TiltEstimatorResource {
            tilt_estimator: TiltEstimator::new(config),
        }
    }
}

#[derive(Resource, Default, Clone, Copy, Debug)]
pub struct DualsenseTilt(TiltEstimates);

impl DualsenseTilt {
    pub fn estimates(&self) -> TiltEstimates {
        self.0
    }
}

fn update_tilt_tilt_system<const SAMPLES: usize>(
    controller: Res<DualsenseController>,
    tilt: ResMut<DualsenseTilt>,
    estimator: ResMut<TiltEstimatorResource<SAMPLES>>,
    time: Res<Time>,
) -> Result<(), BevyError> {
    let state = controller.into_inner().dualsense.lock().unwrap().read()?;
    tilt.into_inner().0 = estimator.into_inner().tilt_estimator.next_estimate(
        &state.accel,
        &state.gyro,
        &time.delta(),
    );

    Ok(())
}

fn handle_results(r: In<Result<(), BevyError>>) -> () {
    match r.0 {
        Ok(()) => (),
        Err(err) => bevy::log::error!("Error in the dualsense-tilt plugin: {}", err),
    }
}

pub struct DualsenseTiltPlugin<const SAMPLES: usize> {
    dualsense: Arc<Mutex<Dualsense>>,
}

impl<const SAMPLES: usize> DualsenseTiltPlugin<SAMPLES> {
    pub fn new(dualsense: Arc<Mutex<Dualsense>>) -> DualsenseTiltPlugin<SAMPLES> {
        DualsenseTiltPlugin { dualsense }
    }
}

impl<const SAMPLES: usize> Plugin for DualsenseTiltPlugin<SAMPLES> {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(DualsenseTilt::default())
            .insert_resource(DualsenseController::new(self.dualsense.clone()))
            .insert_resource(TiltEstimatorResource::new(
                TiltEstimatorConfig::<SAMPLES>::default(),
            ))
            .add_systems(
                Update,
                update_tilt_tilt_system::<SAMPLES>.pipe(handle_results),
            );
    }
}
