use serde_derive::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct DockApp {
    #[serde(rename = "tile-data")]
    pub tile_data: TileData,
}

#[derive(Debug, Deserialize)]
pub struct TileData {
    #[serde(rename = "file-label")]
    pub file_label: String,
}

pub fn read_dock_apps() -> anyhow::Result<Vec<DockApp>> {
    let output = Command::new("defaults")
        .arg("read")
        .arg("com.apple.dock")
        .arg("persistent-apps")
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Unable to run `default` command: {}", stderr);
    }

    Ok(plist::from_bytes(output.stdout.as_slice())?)
}

#[cfg(test)]
mod tests {
    use crate::dock_reader::read_dock_apps;

    #[test]
    fn test_read_dock_apps() {
        let dock_apps = read_dock_apps().unwrap();
        println!("{:?}", dock_apps);

        assert_eq!(dock_apps.is_empty(), false);
    }
}
