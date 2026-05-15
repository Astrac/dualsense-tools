mod emulated;
mod emulated_axis_value;
mod emulator;
mod term_ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    term_ui::run()
    //
    // let mut api = hidapi::HidApi::new()?;
    // let device = Dualsense::new(&mut api)?;
    // let tilt_estimator = TiltEstimator::<10>::new(TiltEstimatorConfig::default());
    //
    // let emulator = Emulator::new(device, tilt_estimator).into_iter();
    //
    // for state in emulator {
    //     println!("{state:?}");
    //     thread::sleep(Duration::from_millis(1000 / 2));
    // }
    //
    // Ok(())
}
