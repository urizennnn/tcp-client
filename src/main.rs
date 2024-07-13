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
        print!("Enter command (LIST, GET <filename>, PUT <filename> DELETE <filename>, or EXIT): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let input = input.trim().to_string();

        if input == "EXIT" {
            println!("Exiting...");
            break;
        }

        if let Err(e) = stream.write_all(input.as_bytes()).await {
            eprintln!("Failed to send command to server: {}", e);
            continue;
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
