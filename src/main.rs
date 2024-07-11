use std::error::Error;
use std::io::{self, BufRead};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:8080").await?;
    println!("Connected to server");

    loop {
        print!("Enter command (LIST, GET <filename>, PUT <filename>, or EXIT): ");

        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let input = input.trim();

        if input == "EXIT" {
            break;
        }

        stream.write_all(input.as_bytes()).await?;
        stream.write_all(b"\n").await?;

        if input.starts_with("GET ") {
            // Implement file download logic
        } else if input.starts_with("PUT ") {
            stream.write_all(b"This is the put request").await?;
            stream.flush().await?;
        } else {
            // For LIST and other commands
            let mut buffer = [0; 1024];
            loop {
                let n = stream.read(&mut buffer).await?;
                if n == 0 {
                    break;
                }
                print!("{}", String::from_utf8_lossy(&buffer[..n]));
            }
        }
    }

    Ok(())
}
