#![warn(rust_2018_idioms)]

mod kafka;
mod serializer;

use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpListener, TcpStream };
use std::error::Error;
use std::str;


async fn tcp_handle_process(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf: [u8; kafka::MAX_BUFFER_SIZE] = [0; kafka::MAX_BUFFER_SIZE];
    loop {
        let n = socket
            .read(&mut buf).await
            .expect("tcp_handle_process::failed_to_read_data_from_socket");
        if n == 0 {
            return Ok(());
        }

        let raw_buffer_data = str::from_utf8(&buf).unwrap();

        println!("tcp_handle_process::received_data: {}", raw_buffer_data);

        let cleaned_buffer: String = serializer::remove_empty_buffer(raw_buffer_data.to_string());
        let serialized_raw = serializer::serialize_message(cleaned_buffer).await;

        match serialized_raw {
            Ok(gps_json_data) => {
                println!("tcp_handle_process::serialize_message::success");
                let kafka_process = kafka::kafka_send_message(gps_json_data).await;

                match kafka_process {
                    Ok(_) => {
                        println!("tcp_handle_process::kafka_send_message::success");
                        socket
                            .write_all(b"+RESP:GBTMI").await
                            .expect("failed to write data to socket");
                    }
                    Err(error) => {
                        println!("tcp_handle_process::kafka_send_message::error: {}", error);
                        socket
                            .write_all(b"+RESP:GBTMI").await
                            .expect("failed to write data to socket");
                    }
                }
            }
            Err(error) => {
                println!("tcp_handle_process::serialize_message::error: {}", error);
            }
        }
        // stream.write_all(&buf[0..n]).await?;
    }
}

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
