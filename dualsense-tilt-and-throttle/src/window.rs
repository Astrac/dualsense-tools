use std::{
    fmt::Display,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::sync::broadcast::Sender;
use eframe::egui::{CentralPanel, Key, ViewportBuilder, ViewportCommand};
use egui_ratatui::RataguiBackend;
use ratatui::Terminal;
use rusttype::Font;
use soft_ratatui::SoftBackend;

use crate::{term_ui::UiState, threads::Command};

#[derive(Clone, Copy, Debug)]
struct FontLoadError;

impl Display for FontLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error loading fonts")
    }
}

impl std::error::Error for FontLoadError {}

pub fn init(
    render_state: Arc<Mutex<UiState>>,
    commands: Sender<Command>,
    frame_duration: Duration,
) -> color_eyre::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1280., 768.]),
        ..Default::default()
    };

    let font_regular =
        Font::try_from_bytes(include_bytes!("../assets/FantasqueSansMono-Regular.otf"))
            .ok_or(FontLoadError)?;
    let font_bold = Font::try_from_bytes(include_bytes!("../assets/FantasqueSansMono-Bold.otf"))
        .ok_or(FontLoadError)?;

    let soft_backend = SoftBackend::<soft_ratatui::EmbeddedTTF>::new(
        60,
        30,
        24,
        font_regular,
        Some(font_bold),
        None,
    );

    let backend = RataguiBackend::new("soft_rat", soft_backend);
    let mut terminal = Terminal::new(backend).unwrap();

    eframe::run_ui_native(
        "Dualsense 6-axis and throttle",
        options,
        move |ctx, _frame| {
            let ppp = ctx.input(|r| r.pixels_per_point());
            if ppp != 1. {
                ctx.set_zoom_factor(1. / ppp);
            }

            CentralPanel::default().show_inside(ctx, |ui| {
                ui.add(terminal.backend_mut());
            });

            let render_state = render_state.lock().unwrap();

            terminal
                .draw(crate::term_ui::render(&render_state))
                .expect("TUI drawing error");

            ctx.request_repaint_after(frame_duration);

            if ctx.input(|r| r.key_pressed(Key::F)) {
                commands.send(Command::NextFeeder).unwrap();
            }

            if ctx.input(|r| r.key_pressed(Key::Q)) {
                commands.send(Command::Quit).unwrap();
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        },
    )?;

    Ok(())
}
