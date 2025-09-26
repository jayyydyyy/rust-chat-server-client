use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

fn main() {
    // minimal login, no config add later
    println!("Please enter the server address");
    let mut server_address = String::new();
    io::stdin().read_line(&mut server_address).expect("Error reading server address from stdin...");
    let server_address = server_address.trim();
    
    println!("Please enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Error reading username!");
    let username = username.trim().to_string();

    match TcpStream::connect(server_address){
        Ok(mut stream) => {
            println!("Successfully connected to the server as {}", username);
            let mut read_stream = stream.try_clone().expect("Failed to clone stream");
            let username_copy = username.clone();
            // Display messages from server
            thread::spawn(move || {
                let mut buffer = [0; 1024];
                loop {
                    match read_stream.read(&mut buffer){
                        Ok(bytes_read) => {
                            if bytes_read == 0{
                                println!("Server Closed Connection!");
                                break;
                            }
                            let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                            if message.trim().starts_with(&format!("{}:", username_copy)){
                                print!("\x1B[1A\x1B[2K{}", message);
                            }else{
                                print!("{}", message);
                            }
                            io::stdout().flush().unwrap();
                        }
                        Err(e) => {
                            eprintln!("Connection to the server was lost. Error: {}", e);
                            break;
                        }
                    }
                }
            });
            // Send input to server
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read from stdin");
                let message = format!("{}: {}", username, input);
                
                if stream.write_all(message.as_bytes()).is_err(){
                    eprintln!("Failed to send message. Is the server running?");
                    break;
                }
            }

        }
        Err(e) => {
            eprintln!("There was an error connecting to server at address: {}. Error: {}", server_address, e);
        }
    }
}

