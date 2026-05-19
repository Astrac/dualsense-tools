use crate::emulated::EmulatedGamepad;
use crossbeam_channel::Receiver;
use ratatui::crossterm::event;
use ratatui::prelude::*;
use ratatui::widgets::{Block, LineGauge, Paragraph};
use std::time::Duration;

const GREEN_BG: Color = Color::Rgb(0, 128, 0);
const RED_BG: Color = Color::Rgb(128, 0, 0);
const RED_BG_TEXT: Color = Color::Rgb(200, 200, 200);

pub fn run(channel: Receiver<EmulatedGamepad>, feeder_description: String) {
    ratatui::run(|terminal| {
        for state in channel {
            terminal.draw(render(state, feeder_description.as_str())).unwrap();
            if event::poll(Duration::from_millis(10)).unwrap()
                && event::read().unwrap().is_key_press()
            {
                break;
            }
        }
    });
}

fn render(state: EmulatedGamepad, feeder_description: &str) -> impl FnMut(&mut Frame) {
    move |frame| {
        let main_areas = [
            Constraint::Fill(10),  // Spacer
            Constraint::Length(3), // Title
            Constraint::Min(11),   // Axes
            Constraint::Min(5),    // Buttons
            Constraint::Min(3),    // Feeder Info
            Constraint::Length(1), // Footer
            Constraint::Fill(10),  // Spacer
        ];

        let layout = Layout::vertical(main_areas).spacing(1).horizontal_margin(8);
        let [
            _,
            title_area,
            axes_area,
            buttons_area,
            feeder_info_area,
            footer_area,
            _,
        ] = frame.area().layout(&layout);

        render_title(frame, title_area);
        render_axes(frame, axes_area, state);
        render_buttons(frame, buttons_area, state);
        render_footer(frame, footer_area);
        render_feeder_info(frame, feeder_description, feeder_info_area);
    }
}

fn render_feeder_info(
    frame: &mut Frame<'_>,
    feeder_description: &str,
    feeder_info_area: Rect,
) {
    frame.render_widget(
        Block::bordered().title("Emulated Gamepad Feeder"),
        feeder_info_area,
    );
    frame.render_widget(
        Paragraph::new(feeder_description).centered().bold(),
        feeder_info_area.centered_vertically(Constraint::Length(1)),
    );
}

fn render_title(frame: &mut Frame, title_area: Rect) {
    let title = Line::from_iter([Span::from("Dualsense Tilt And Throttle Emulator")
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

fn render_buttons(frame: &mut Frame, buttons_area: Rect, state: EmulatedGamepad) {
    frame.render_widget(Block::bordered().title("Emulated Buttons"), buttons_area);

    let buttons_constraints = [Constraint::Length(11); 13];
    let buttons_layout = Layout::horizontal(buttons_constraints).flex(layout::Flex::SpaceBetween);
    let buttons_cells: [_; 13] = buttons_area
        .inner(Margin::new(2, 2))
        .layout(&buttons_layout);

    for (i, cell) in buttons_cells.iter().enumerate() {
        if i < state.buttons.len() {
            let button_fg = if state.buttons[i] { GREEN_BG } else { RED_BG };

            frame.render_widget(
                Paragraph::new(format!("Button {i:02}"))
                    .bg(button_fg)
                    .fg(RED_BG_TEXT)
                    .centered(),
                *cell,
            );
        }
    }
}

fn render_axes(frame: &mut Frame, axes_area: Rect, state: EmulatedGamepad) {
    let axes_rows = [Constraint::Length(6), Constraint::Length(2)];

    let [axes, throttle] = axes_area
        .centered_vertically(Constraint::Length(8))
        .layout(&Layout::vertical(axes_rows).spacing(0));

    let axes_cols = [Constraint::Fill(1); 2];
    let [left, right] = axes.layout(&Layout::horizontal(axes_cols).spacing(0));

    let axes_gauges_rows = [Constraint::Length(2); 3];
    let axes_layout = Layout::vertical(axes_gauges_rows).spacing(0);

    let [axis_00, axis_01, axis_02] = left.layout(&axes_layout);
    let [axis_10, axis_11, axis_12] = right.layout(&axes_layout);

    frame.render_widget(Block::bordered().title("Emulated Axes"), axes_area);
    render_line_gauge(frame, axis_00, "Roll", state.axes.roll.as_i8());
    render_line_gauge(frame, axis_01, "Pitch", state.axes.pitch.as_i8());
    render_line_gauge(frame, axis_02, "RX", state.axes.rx.as_i8());
    render_line_gauge(frame, axis_10, "X", state.axes.x.as_i8());
    render_line_gauge(frame, axis_11, "Y", state.axes.y.as_i8());
    render_line_gauge(frame, axis_12, "RY", state.axes.ry.as_i8());
    render_line_gauge(frame, throttle, "Throttle", state.axes.throttle.as_i8());
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
