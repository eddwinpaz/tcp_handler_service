#![warn(rust_2018_idioms)]

use std::error::Error;
use tokio::net::{ TcpListener };

mod handler;
mod kafka;
mod serializer;
mod model;

use crate::handler::tcp_handle_process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let port = env::var("SERVER_PORT").is_ok();
    // let port = match port {
    //     true => env::var("SERVER_PORT").unwrap(),
    //     false => "7011".to_string(),
    // };
    let host = String::from("0.0.0.0");
    let addr = format!("{}:7011", host);

    let listener = TcpListener::bind(&addr).await?;
    println!("Server is listening: {}", addr);

    let (socket, _) = listener.accept().await?;
    tcp_handle_process(socket).await?;
    Ok(())
}