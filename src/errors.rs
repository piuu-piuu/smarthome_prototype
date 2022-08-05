use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmartHouseError {
    #[error("No device name provided")]
    NoDeviceName,
    #[error("No device found")]
    NoDeviceFound,
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("TCP connection error")]
    TcpConnectionError,
}
