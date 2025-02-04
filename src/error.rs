use thiserror::Error;

#[derive(Error, Debug)]
pub enum NvidiaError {
    #[error("Registry access failed")]
    RegistryError,
    #[error("Driver download failed")]
    DownloadError,
    #[error("Checksum mismatch")]
    ChecksumMismatch,
    #[error("Installation failed")]
    InstallationError,
    #[error("Hardware detection failed")]
    HardwareDetectionError,
}