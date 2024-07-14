use std::error::Error;
use std::io::{self, BufRead, Write};
use tcp_client::methods::upload::upload;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:8080").await?;
    println!("Connected to server");
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
            _ => {
                stream.write_all(b"\n").await?;
                stream.flush().await?;
            }
        }

        match stream.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                let response = String::from_utf8_lossy(&buffer[..n]);
                println!("Server response: {}", response);
            }
            Ok(_) => {
                println!("Connection closed by server");
                break;
            }
            Err(e) => {
                eprintln!("Failed to read from server: {}", e);
                break;
            }
        }
    }
    Ok(())
}
