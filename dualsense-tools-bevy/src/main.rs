mod plugin;
mod scene;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    scene::scene::<10>();
    Ok(())
}
