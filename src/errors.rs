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
    #[error("TCP write error")]
    TcpWriteError,
    #[error("TCP read error")]
    TcpReadError,
    #[error("UDP connection error")]
    UdpConnectionError,
    #[error("UDP write error")]
    UdpWriteError,
    #[error("UDP read error")]
    UdpReadError,
}
