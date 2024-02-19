use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the address to bind to
    let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;

    // Bind the listener to the specified address
    let mut listener = TcpListener::bind(&addr).await?;
    println!("Server running on {}", addr);

    // Start accepting incoming connections
    while let Ok((mut socket, _)) = listener.accept().await {
        // Handle each connection in a separate task
        tokio::spawn(async move {
            if let Err(e) = handle_connection(&mut socket).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(socket: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // Buffer to read data from the socket
    let mut buf = [0; 1024];
    loop {
        // Read data from the socket
        let n = match socket.read(&mut buf).await {
            // Handle successful read
            Ok(n) if n == 0 => return Ok(()),
            Ok(n) => n,
            // Handle read error
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return Err(e.into());
            }
        };

        // Echo the received data back to the client
        if let Err(e) = socket.write_all(&buf[..n]).await {
            eprintln!("Failed to write to socket: {}", e);
            return Err(e.into());
        }
    }
}
