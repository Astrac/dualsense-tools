mod emulated;
mod emulated_axis_value;
mod emulator;
mod term_ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    term_ui::run()
}
