use std::io::{
    Read,
    prelude::*,
    ErrorKind,
    BufReader
};
use std::net::{ Shutdown, TcpStream };

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:33100").unwrap();
    handle(&stream).await

} // the stream is closed here

async fn handle(mut stream: &TcpStream) -> std::io::Result<()> {
    Ok(())
}

async fn default_handle( mut stream: &TcpStream ) -> std::io::Result<()> {
    let message = "Hi Patty!\nH\nsome_handle\nsome_handle\nthis should not be reached\n".to_string().into_bytes();
    stream.write(&message).unwrap();
    stream.flush().unwrap();
    println!("requester listening");
    let mut buf_reader: [u8; 128] = [0; 128];
    let tcp_request= stream.read(&mut buf_reader).unwrap();

    println!("Request: {:#?}", tcp_request);
    println!("Buffer: {:#?}", String::from_utf8(buf_reader.to_vec()) );
    Ok(())
}