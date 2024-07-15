use std::error::Error;
use std::io::{self, BufRead, Write};
use tcp_client::init;
use tcp_client::methods::upload::upload;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:8080").await?;
    println!("Connected to server");
    init::init().await?;
    let mut buffer = vec![0; 5_242_880];

    loop {
        print!(
            "Enter command (LIST, GET <filename>, PUT <filename>, DELETE <filename>, or EXIT): "
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let input = input.trim().to_string();

        match input.split_whitespace().next() {
            Some("EXIT") => {
                println!("Exiting");
                break;
            }
            Some("PUT") => {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() != 2 {
                    println!("Invalid PUT command. Usage: PUT <filename>");
                    continue;
                }
                stream.write_all(input.as_bytes()).await?;
                stream.flush().await?;
                upload(&mut stream, parts[1], &mut buffer).await?;
            }
            Some("LIST") | Some("GET") | Some("DELETE") => {
                stream.write_all(input.as_bytes()).await?;
                stream.flush().await?;

                let mut full_response = String::new();
                loop {
                    match stream.read(&mut buffer).await {
                        Ok(0) => break, // Connection closed
                        Ok(n) => {
                            full_response.push_str(&String::from_utf8_lossy(&buffer[..n]));
                            if n < buffer.len() {
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from server: {}", e);
                            return Err(e.into());
                        }
                    }
                }
                println!("Server response:\n{}", full_response.trim());
            }
            _ => {
                println!("Invalid command. Please try again.");
            }
        }
    }
    Ok(())
}
