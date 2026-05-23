use crate::emulated::EmulatedGamepad;
use ratatui::prelude::*;
use ratatui::widgets::{Block, LineGauge, Paragraph};

const GREEN_BG: Color = Color::Rgb(0, 128, 0);
const RED_BG: Color = Color::Rgb(128, 0, 0);
const RED_BG_TEXT: Color = Color::Rgb(200, 200, 200);

pub fn render(state: EmulatedGamepad, feeder_description: &str) -> impl FnMut(&mut Frame) {
    move |frame| {
        frame.render_widget(Block::default().bg(Color::Rgb(20, 20, 40)), frame.area());

        let main_areas = [
            Constraint::Fill(10),  // Spacer
            Constraint::Length(3), // Title
            Constraint::Min(11),   // Axes
            Constraint::Min(9),    // Buttons and dpad
            Constraint::Length(1), // Footer
            Constraint::Fill(10),  // Spacer
        ];

        let layout = Layout::vertical(main_areas).spacing(1).horizontal_margin(8);
        let [
            _,
            title_area,
            axes_area,
            buttons_and_feeder_area,
            footer_area,
            _,
        ] = frame.area().layout(&layout);

        render_title(frame, title_area);
        render_axes(frame, axes_area, &state);
        render_buttons_and_feeder_info(frame, buttons_and_feeder_area, feeder_description, &state);
        render_footer(frame, footer_area);
    }
}

fn render_feeder_info(
    frame: &mut Frame<'_>,
    feeder_description: &str,
    tilt_enabled: bool,
    feeder_info_area: Rect,
) {
    let tilt_state = if tilt_enabled { "Enabled" } else { "Disabled" };

    frame.render_widget(
        Block::bordered().title("Emulated Gamepad Feeder"),
        feeder_info_area,
    );
    frame.render_widget(
        Paragraph::new(format!(
            "Virtual device feeder: {feeder_description}\nTilt emulation: {tilt_state}"
        )),
        feeder_info_area.inner(Margin::new(3, 2)),
    );
}

fn render_title(frame: &mut Frame, title_area: Rect) {
    let title = Line::from_iter([Span::from("Dualsense 6-axis and throttle emulator")
        .bold()
        .fg(RED_BG_TEXT)]);

    frame.render_widget(Block::default().bg(RED_BG), title_area);
    frame.render_widget(
        title.centered(),
        title_area.centered_vertically(Constraint::Length(1)),
    );
}

fn render_footer(frame: &mut Frame, footer_area: Rect) {
    frame.render_widget(Block::default(), footer_area);
    frame.render_widget(
        Paragraph::new("For information please visit: http://github.com/Astrac/dualsense-tools")
            .centered()
            .bold(),
        footer_area.centered_vertically(Constraint::Length(1)),
    );
}

fn render_buttons_and_feeder_info(
    frame: &mut Frame,
    area: Rect,
    feeder_description: &str,
    state: &EmulatedGamepad,
) {
    let columns = [Constraint::Length(63), Constraint::Min(30)];

    let [buttons_area, dpad_area] = area.layout(&Layout::horizontal(columns).spacing(1));

    render_buttons(frame, buttons_area, state);
    render_feeder_info(frame, feeder_description, state.is_tilt_enabled, dpad_area);
}

fn render_buttons(frame: &mut Frame, area: Rect, state: &EmulatedGamepad) {
    frame.render_widget(Block::bordered().title("Emulated Buttons"), area);

    let vertical = Layout::vertical([Constraint::Length(1); 3]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Length(13); 5]).spacing(1);
    let buttons_cells = area
        .inner(Margin::new(2, 2))
        .layout_vec(&vertical)
        .into_iter()
        .flat_map(|row| row.layout_vec(&horizontal));

    for (i, cell) in buttons_cells.enumerate() {
        if i < state.buttons.len() {
            let button_fg = if state.buttons[i] { GREEN_BG } else { RED_BG };

            frame.render_widget(
                Paragraph::new(format!("Button {i:02}"))
                    .bg(button_fg)
                    .fg(RED_BG_TEXT)
                    .centered(),
                cell,
            );
        }
    }
}

fn render_axes(frame: &mut Frame, axes_area: Rect, state: &EmulatedGamepad) {
    let axes_cols = [Constraint::Fill(1); 2];
    let [left, right] = axes_area
        .inner(Margin::new(0, 2))
        .layout(&Layout::horizontal(axes_cols).spacing(0));

    let axes_gauges_rows = [Constraint::Length(2); 4];
    let axes_layout = Layout::vertical(axes_gauges_rows).spacing(1);

    let [axis_00, axis_01, axis_02, throttle] = left.layout(&axes_layout);
    let [axis_10, axis_11, axis_12, hat] = right.layout(&axes_layout);

    frame.render_widget(Block::bordered().title("Emulated Axes"), axes_area);
    render_line_gauge(frame, axis_00, "Roll", state.axes.roll.as_i8());
    render_line_gauge(frame, axis_01, "Pitch", state.axes.pitch.as_i8());
    render_line_gauge(frame, axis_02, "RX", state.axes.rx.as_i8());
    render_line_gauge(frame, axis_10, "X", state.axes.x.as_i8());
    render_line_gauge(frame, axis_11, "Y", state.axes.y.as_i8());
    render_line_gauge(frame, axis_12, "RY", state.axes.ry.as_i8());
    render_line_gauge(frame, throttle, "Throttle", state.axes.throttle.as_i8());

    frame.render_widget(
        Paragraph::new(format!("Hat status: {}", state.hat)),
        hat.inner(Margin::new(2, 0)),
    );
}

pub fn render_line_gauge(frame: &mut Frame, area: Rect, label: &str, value: i8) {
    let ratio: f64 = 0.5 + ((value as f64) / (i8::MAX as f64 - i8::MIN as f64));
    let line_gauge = LineGauge::default()
        .filled_style(Style::new().fg(RED_BG_TEXT).bg(RED_BG).bold())
        .unfilled_style(Style::new().gray().on_black())
        .label(format!("{label} [{value:+04}]"))
        .ratio(ratio.clamp(0.0, 1.0))
        .filled_symbol(symbols::line::THICK_HORIZONTAL)
        .unfilled_symbol(symbols::line::THICK_HORIZONTAL);
    frame.render_widget(line_gauge, area.inner(Margin::new(2, 0)));
}
