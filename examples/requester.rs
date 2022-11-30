use async_std::net::{ TcpStream };
use custom_noise::noise::NoiseInstance;

#[async_std::main]
async fn main() {
    static SECRET: &[u8; 13] = b"we care a lot";
    let async_stream = TcpStream::connect("127.0.0.1:33100").await.unwrap();
    let noise_instance = custom_noise::noise::NoiseInstance::initiator_from_secret(async_stream, SECRET);
    handle_handshake(noise_instance).await;
    println!("Session established ?");
    println!("connection closed.");

} // the stream is closed here

async fn handle_handshake(mut noise_instance: NoiseInstance) {
    noise_instance.handshake_send(&[]).await;
    noise_instance.handshake_listen().await;
    noise_instance.handshake_send(&[]).await;
    
    // noise_instance.transport_send(b"Hello World");
    let msg = b"we really care";
    noise_instance.transport_send(msg).await;
    // println!("client said: {}", String::from_utf8_lossy(&noise_instance.buf[..len]));
    
}

// async fn default_handle( mut stream: &TcpStream ) -> std::io::Result<()> {
//     let message = "Hi Patty!\nH\nsome_handle\nsome_handle\nthis should not be reached\n".to_string().into_bytes();
//     stream.write(&message).unwrap();
//     stream.flush().unwrap();
//     println!("requester listening");
//     let mut buf_reader: [u8; 128] = [0; 128];
//     let tcp_request= stream.read(&mut buf_reader).unwrap();

//     println!("Request: {:#?}", tcp_request);
//     println!("Buffer: {:#?}", String::from_utf8(buf_reader.to_vec()) );
//     Ok(())
// }