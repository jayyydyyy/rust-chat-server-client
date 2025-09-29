use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const SERVER_ADDRESS: &str = "0.0.0.0:4321";

fn main() {
    // Bind listener
    let listener = TcpListener::bind(SERVER_ADDRESS).expect("Failed to bind to address!");
    println!("✅ SERVER LISTENING ON AT {}", SERVER_ADDRESS);

    // Thread safe list
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                println!("New connection! : {}", stream.peer_addr().unwrap());
                let clients_clone = Arc::clone(&clients);

                thread::spawn(move || {
                    handle_client(stream, clients_clone);
                });
            }
            Err(e) => {
                eprintln!("Error accepting the connection! :( {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>){
    clients.lock().unwrap().push(stream.try_clone().expect("Failed to clone stream!"));
    
    let mut buffer = [0; 1024];
    let client_addr = stream.peer_addr().unwrap();
    loop {
        match stream.read(&mut buffer){
            Ok(bytes_read) => {
                // If no bytes, then theyre gone!
                if bytes_read == 0 {
                    println!("Client disconnected: {}", client_addr);
                    break;
                }

                // this means we do have bytes!
                let message = &buffer[0..bytes_read];
                println!("Received message, broadcasting...");

                // Lock the clients and broadcast!
                let mut clients_guard = clients.lock().unwrap();
                for client in clients_guard.iter_mut() {
                    client.write_all(message).unwrap_or_else(|err|{
                        eprintln!("Failed to send message: {}", err);
                    })
                }
            }
            Err(_e) => {
                eprintln!("Error occurred, terminating connection with {}", client_addr);
                break;
            }
        }
    }
    remove_client(client_addr, clients);
}

fn remove_client(client_addr: SocketAddr, clients: Arc<Mutex<Vec<TcpStream>>>){
    let mut clients_guard = clients.lock().unwrap();
    clients_guard.retain(|client|{
        if let Ok(addr) = client.peer_addr(){
            addr != client_addr
        }else{
            // Can't get address, return false
            false
        }
    });
    println!("Removed client with addr: {}", client_addr);
}