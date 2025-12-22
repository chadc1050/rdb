use crate::parser::Parser;
use tokio::{io::AsyncReadExt, net::TcpListener};

pub mod parser;

#[tokio::main]
async fn main() {
    println!("Starting database...");
    let listener = TcpListener::bind("127.0.0.1:6543")
        .await
        .expect("could not bind to port 6543");

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                tokio::spawn(async move {
                    println!("Recieved connection: {}.", addr.ip());

                    let mut buffer = [0u8; 1024];

                    let size = socket
                        .peek(&mut buffer)
                        .await
                        .expect("could not get size of message.");

                    let mut payload = vec![0u8; size];
                    socket.read_exact(&mut payload).await.unwrap();

                    println!("Recieved: {}", String::from_utf8(payload).unwrap());
                });
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn process_request(payload: &[u8]) {
    let parser = Parser::new(&payload);
    parser.parse();
}
