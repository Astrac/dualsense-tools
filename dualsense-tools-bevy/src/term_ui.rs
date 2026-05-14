use dualsense_tools::{Dualsense, state::DualsenseState, SENSORS_MIN, SENSORS_RANGE};
use ratatui::crossterm::event;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::LineGauge,
};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[allow(dead_code)]
pub fn run(ds: Arc<Mutex<Dualsense>>) -> color_eyre::Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| {
        loop {
            let state = ds.lock().unwrap().read()?;
            terminal.draw(render(state))?;
            if event::poll(Duration::from_millis(10))? {
                if event::read()?.is_key_press() {
                    break Ok(());
                }
            }
        }
    })
}

#[allow(dead_code)]
fn render(state: DualsenseState) -> impl FnMut(&mut Frame) {
    move |frame| {
        let main_areas = [
            Constraint::Length(1), // Title
            Constraint::Max(5),    // Body
            Constraint::Fill(1),   // Spacer
        ];

        let layout = Layout::vertical(main_areas).spacing(1);
        let [top, second, _] = frame.area().layout(&layout);

        let content_columns = [Constraint::Fill(1), Constraint::Fill(1)];
        let content_layout = Layout::horizontal(content_columns).spacing(1);
        let [left, right] = second.layout(&content_layout);

        let axes_gauges_rows = [Constraint::Fill(1); 3];
        let axes_layout = Layout::vertical(axes_gauges_rows)
            .spacing(1)
            .horizontal_margin(10);

        let [accel_x, accel_y, accel_z] = left.layout(&axes_layout);
        let [gyro_x, gyro_y, gyro_z] = right.layout(&axes_layout);

        let title = Line::from_iter([Span::from("Dualsense Status").bold()]);
        frame.render_widget(title.centered(), top);

        render_line_gauge(frame, accel_x, "Accel X", state.accel.x.as_i16());
        render_line_gauge(frame, accel_y, "Accel Y", state.accel.y.as_i16());
        render_line_gauge(frame, accel_z, "Accel Z", state.accel.z.as_i16());
        render_line_gauge(frame, gyro_x, "Gyro X", state.gyro.x.as_i16());
        render_line_gauge(frame, gyro_y, "Gyro Y", state.gyro.y.as_i16());
        render_line_gauge(frame, gyro_z, "Gyro Z", state.gyro.z.as_i16());
    }
}

/// Render a line gauge (compact progress bar).
pub fn render_line_gauge(frame: &mut Frame, area: Rect, label: &str, value: i16) {
    let norm_value: f64 = (value - SENSORS_MIN) as f64;
    let ratio: f64 = norm_value / (SENSORS_RANGE as f64);
    let line_gauge = LineGauge::default()
        .filled_style(Style::new().white().on_red().bold())
        .unfilled_style(Style::new().gray().on_black())
        .label(label)
        .ratio(ratio.clamp(0.0, 1.0))
        .filled_symbol(symbols::line::THICK_HORIZONTAL)
        .unfilled_symbol(symbols::line::THICK_HORIZONTAL);
    frame.render_widget(line_gauge, area);
}
