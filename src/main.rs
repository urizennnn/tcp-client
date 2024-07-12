use std::error::Error;
use std::io::{self, BufRead, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:8080").await?;
    println!("Connected to server");

    let mut buffer = vec![0; 1024];

    loop {
        // Display prompt to the user
        print!("Enter command (LIST, GET <filename>, PUT <filename>, or EXIT): ");
        io::stdout().flush()?;

        // Read user input
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let input = input.trim().to_string();

        if input == "EXIT" {
            break;
        }

        // Send user input to the server
        stream.write_all(input.as_bytes()).await?;
        stream.write_all(b"\n").await?;

        // Read response from the server after sending the command
        match stream.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                println!("Server response: {}", String::from_utf8_lossy(&buffer[..n]));
            }
            Ok(_) => {
                println!("Connection closed by server");
                break;
            }
            Err(e) => {
                eprintln!("Failed to read from server: {:?}", e);
                return Err(e.into());
            }
        }
    }

    Ok(())
}
