use std::path::PathBuf;
use sha2::{Sha256, Digest};
use windows::Win32::System::Registry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct NvidiaGpu {
    pub model: String,
    pub device_id: u32,
    pub driver_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DriverManifest {
    versions: Vec<DriverVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DriverVersion {
    version: String,
    url: String,
    checksum: String,
    supported_devices: Vec<u32>,
}

#[derive(thiserror::Error, Debug)]
pub enum DriverError {
    #[error("GPU detection failed")]
    DetectionFailure,
    #[error("Network error")]
    NetworkError(#[from] reqwest::Error),
    #[error("Installation failed")]
    InstallFailure,
}

pub async fn detect_gpu() -> Result<NvidiaGpu, DriverError> {
    let gpu_info = unsafe { detect_nvidia_hardware() }?;
    Ok(gpu_info)
}

pub async fn download_driver(gpu: &NvidiaGpu) -> Result<PathBuf, DriverError> {
    let manifest = fetch_driver_manifest().await?;
    let target_version = select_driver_version(&manifest, gpu.device_id)?;
    let temp_path = download_file(&target_version.url).await?;
    verify_checksum(&temp_path, &target_version.checksum)?;
    Ok(temp_path)
}

async fn fetch_driver_manifest() -> Result<DriverManifest, DriverError> {
    let client = reqwest::Client::new();
    let response = client.get("https://api.nvidia.com/v1/drivers")
        .send()
        .await?;
    Ok(response.json().await?)
}

unsafe fn detect_nvidia_hardware() -> Result<NvidiaGpu, DriverError> {
    let mut hkey = Registry::HKEY::default();
    let key_path = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}";
    let mut device_id = 0u32;
    
    Registry::RegOpenKeyExW(
        Registry::HKEY_LOCAL_MACHINE,
        key_path,
        0,
        Registry::KEY_READ,
        &mut hkey,
    )?;

    let mut buffer = [0u16; 1024];
    let mut size = 1024u32;
    
    Registry::RegQueryValueExW(
        hkey,
        "DriverDesc",
        None,
        None,
        Some(&mut buffer.as_mut_ptr() as *mut _),
        Some(&mut size),
    )?;

    let model = String::from_utf16_lossy(&buffer[..size as usize/2]);
    Ok(NvidiaGpu {
        model: model.trim_end_matches('\0').to_string(),
        device_id: 0x1C03,
        driver_version: "516.94".into(),
    })
}