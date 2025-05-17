mod app_launcher;
mod dock_reader;
mod key_grabber;

fn main() -> anyhow::Result<()> {
    key_grabber::start()?;
    Ok(())
}
