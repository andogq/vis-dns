use std::net::UdpSocket;

use thiserror::Error;

#[derive(Error, Debug)]
enum VisDnsError {
    #[error("Unknown IO Error")]
    Io(#[from] std::io::Error),

    #[error("Unable to bind to address")]
    Bind(std::io::Error),
}

fn main() -> Result<(), VisDnsError> {
    let socket = UdpSocket::bind("1.1.1.1:53").map_err(VisDnsError::Bind)?;

    Ok(())
}
