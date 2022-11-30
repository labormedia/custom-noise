use async_std::net::{ TcpStream };
use custom_noise::noise::NoiseInstance;

#[async_std::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
    if &args.len() < &3 {
        usage_message(); 
        panic!("Expected parameters")
    }
    let address = &[&args[1], ":", &args[2]].concat();

    static SECRET: &[u8; 13] = b"do care a lot";
    // let address = "127.0.0.1:33100";
    println!("Connecting to address : {:?}", address);
    let async_stream = TcpStream::connect(address).await.unwrap();
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

fn usage_message() {
    println!("
    Usage: ./target/debug/examples/requester [IP] [PORT]
    ")
}