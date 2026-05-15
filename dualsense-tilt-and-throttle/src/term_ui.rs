use crate::emulated::EmulatedGamepad;
use crate::emulator::Emulator;
use dualsense_tools::Dualsense;
use dualsense_tools::{TiltEstimator, TiltEstimatorConfig};
use ratatui::crossterm::event;
use ratatui::prelude::*;
use ratatui::widgets::LineGauge;
use std::time::Duration;

#[allow(dead_code)]
pub fn run() -> color_eyre::Result<()> {
    let mut api = hidapi::HidApi::new()?;
    let device = Dualsense::new(&mut api)?;
    let tilt_estimator = TiltEstimator::<10>::new(TiltEstimatorConfig::default());

    let emulator = Emulator::new(device, tilt_estimator).into_iter();

    ratatui::run(|terminal| {
        for state in emulator {
            terminal.draw(render(state)).unwrap();
            if event::poll(Duration::from_millis(10)).unwrap()
                && event::read().unwrap().is_key_press()
            {
                break;
            }
        }
    });

    Ok(())
}

#[allow(dead_code)]
fn render(state: EmulatedGamepad) -> impl FnMut(&mut Frame) {
    move |frame| {
        let main_areas = [
            Constraint::Length(1), // Title
            Constraint::Fill(10),  // Body
            Constraint::Fill(1),   // Spacer
        ];

        let layout = Layout::vertical(main_areas).spacing(1);
        let [top, second, _] = frame.area().layout(&layout);

        let content_columns = [Constraint::Fill(1), Constraint::Fill(1)];
        let content_layout = Layout::horizontal(content_columns).spacing(1);
        let [left, right] = second.layout(&content_layout);

        let axes_gauges_rows = [Constraint::Fill(1); 4];
        let axes_layout = Layout::vertical(axes_gauges_rows)
            .spacing(1)
            .horizontal_margin(10);

        let [accel_x, accel_y, accel_z, _] = left.layout(&axes_layout);
        let [gyro_x, gyro_y, gyro_z, gyro_zz] = right.layout(&axes_layout);

        let title = Line::from_iter([Span::from("Dualsense Status").bold()]);
        frame.render_widget(title.centered(), top);

        render_line_gauge(frame, accel_x, "Roll", state.axes.roll.as_i8());
        render_line_gauge(frame, accel_y, "Pitch", state.axes.pitch.as_i8());
        render_line_gauge(frame, accel_z, "Throttle", state.axes.throttle.as_i8());
        render_line_gauge(frame, gyro_x, "X", state.axes.x.as_i8());
        render_line_gauge(frame, gyro_y, "Y", state.axes.y.as_i8());
        render_line_gauge(frame, gyro_z, "RX", state.axes.rx.as_i8());
        render_line_gauge(frame, gyro_zz, "RY", state.axes.ry.as_i8());
    }
}

/// Render a line gauge (compact progress bar).
pub fn render_line_gauge(frame: &mut Frame, area: Rect, label: &str, value: i8) {
    let ratio: f64 = 0.5 + ((value as f64) / (i8::MAX as f64 - i8::MIN as f64));
    let line_gauge = LineGauge::default()
        .filled_style(Style::new().white().on_red().bold())
        .unfilled_style(Style::new().gray().on_black())
        .label(format!("{label} - {value}"))
        .ratio(ratio.clamp(0.0, 1.0))
        .filled_symbol(symbols::line::THICK_HORIZONTAL)
        .unfilled_symbol(symbols::line::THICK_HORIZONTAL);
    frame.render_widget(line_gauge, area);
}
