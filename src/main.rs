use once_cell::sync::Lazy;
use std::{
    error::Error,
    io::{self, BufRead, Write},
};
use tcp_client::{
    init,
    methods::{list::list, upload::upload},
};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use whoami::username;

pub static USER: Lazy<String> = Lazy::new(|| username().to_string());

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:8080").await?;
    stream.write_all(USER.as_bytes()).await?;
    stream.flush().await?;
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
                list(&mut stream, &mut buffer).await?;
            }
            _ => {
                println!("Invalid command. Please try again.");
            }
        }
    }
    Ok(())
}
