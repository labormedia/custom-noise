use async_std::net::{ TcpStream };
use custom_noise::noise::NoiseInstance;

#[async_std::main]
async fn main() {
    static SECRET: &[u8; 13] = b"do care a lot";
    let async_stream = TcpStream::connect("127.0.0.1:33100").await.unwrap();
    let noise_instance = custom_noise::noise::NoiseInstance::initiator_from_secret(async_stream, SECRET);
    initiate_nn_handshake(noise_instance).await;
    println!("Session established.");
    println!("connection closed.");

} // the stream is closed here

async fn initiate_nn_handshake(mut noise_instance: NoiseInstance) {
    noise_instance.handshake_send(&[]).await;
    noise_instance.handshake_listen().await;
    noise_instance.handshake_send(&[]).await;

    let msg = b"we really care";
    noise_instance.transport_send(msg).await;
    println!("Sent : {}", String::from_utf8_lossy(msg));
}