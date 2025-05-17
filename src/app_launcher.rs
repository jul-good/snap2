use std::process::Command;

pub fn launch_app(name: &str) -> anyhow::Result<()> {
    Command::new("open")
        .args(["-a", name])
        .spawn()?;
    
    Ok(())
}
