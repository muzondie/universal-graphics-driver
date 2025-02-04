use reqwest::blocking::Client;
use std::path::PathBuf;
use thiserror::Error;
use zip::ZipArchive;

#[derive(Error, Debug)]
pub enum DriverError {
    #[error("Driver package not found for {0}")]
    NotFound(String),
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    #[error("Installation failed")]
    InstallationFailed,
}

pub struct DriverPackage {
    pub vendor: String,
    pub download_url: String,
    pub checksum: String,
    pub supported_os: Vec<String>,
}

pub fn load_driver_database() -> std::collections::HashMap<String, DriverPackage> {
    let mut db = std::collections::HashMap::new();
    db.insert("NVIDIA".into(), DriverPackage {
        vendor: "NVIDIA".into(),
        download_url: "https://driver.nvidia.com/latest".into(),
        checksum: "nvidia_sha256".into(),
        supported_os: vec!["Windows 10".into(), "Windows 11".into()],
    });
    db
}

pub fn download_and_install(pkg: &DriverPackage) -> Result<(), DriverError> {
    let client = Client::new();
    let response = client.get(&pkg.download_url)
        .send()
        .map_err(|e| DriverError::DownloadFailed(e.to_string()))?;

    let temp_dir = std::env::temp_dir();
    let zip_path = temp_dir.join("driver.zip");
    
    std::fs::write(&zip_path, response.bytes().map_err(|e| DriverError::DownloadFailed(e.to_string()))?)
        .map_err(|_| DriverError::InstallationFailed)?;

    let file = std::fs::File::open(&zip_path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    archive.extract(&temp_dir).unwrap();

    Ok(())
}

fn execute_installer(path: &PathBuf) -> Result<(), DriverError> {
    std::process::Command::new(path)
        .arg("/S")
        .status()
        .map(|_| ())
        .map_err(|_| DriverError::InstallationFailed)
}