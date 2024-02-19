use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener as TokioTcpListener;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

type Clients = Arc<Mutex<HashMap<SocketAddr, Sender<String>>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the address to bind to
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // Check if the address is already in use
    if is_address_in_use(&addr) {
        eprintln!("Address {} is already in use", addr);
        return Ok(());
    }

    // Create a shared state for storing client channels
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // Bind the listener to the specified address
    let listener = TokioTcpListener::bind(&addr).await?;
    println!("Server running on {}", addr);

    // Start accepting incoming connections
    while let Ok((mut socket, client_addr)) = listener.accept().await {
        // Clone the clients map for use in the task
        let clients_clone = clients.clone();

        // Create a channel to communicate with the client
        let (tx, mut rx) = mpsc::channel::<String>(10);
        {
            // Lock the clients map and add the client's channel sender
            let mut clients_map = clients.lock().unwrap();
            clients_map.insert(client_addr, tx);
        }

        // Handle each connection in a separate task
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => break,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        break;
                    }
                };

                // Broadcast the received message to all other clients
                let received_message = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                broadcast(&clients_clone, &client_addr, &received_message).await;

                // Clear the buffer for the next message
                buf = [0; 1024];
            }

            // Remove client from the clients map
            let mut clients_map = clients_clone.lock().unwrap();
            clients_map.remove(&client_addr);
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

async fn broadcast(clients: &Clients, sender_addr: &SocketAddr, message: &str) {
    let mut clients_map = clients.lock().unwrap();
    for (client_addr, tx) in clients_map.iter_mut() {
        if *client_addr != *sender_addr {
            if let Err(_) = tx.send(message.to_string()).await {
                eprintln!("Failed to send message to client {}", client_addr);
            }
        }
    }
}
