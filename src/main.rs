use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener as TokioTcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the address to bind to
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // Check if the address is already in use
    if is_address_in_use(&addr) {
        eprintln!("Address {} is already in use", addr);
        return Ok(());
    }

    // Bind the listener to the specified address
    let mut listener = TokioTcpListener::bind(&addr).await?;
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

fn is_address_in_use(addr: &SocketAddr) -> bool {
    if let Ok(listener) = std::net::TcpListener::bind(addr) {
        // The address is not in use
        drop(listener);
        false
    } else {
        // The address is in use
        true
    }
}

async fn handle_connection(socket: &mut tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // Implement your connection handling logic here
    Ok(())
}
