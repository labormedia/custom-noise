use async_std::net::{TcpListener};
use custom_noise::noise::NoiseInstance;
use async_std::stream::StreamExt;

#[async_std::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
    if &args.len() < &3 {
        usage_message(); 
        panic!("Expected parameters")
    }

    // let local_address = "127.0.0.1:33100";
    let address = &[&args[1], ":", &args[2]].concat();

    static SECRET: &[u8] = b"we care a lot";
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Listening on address : {:?}", address);

    loop {
        match listener.incoming().next().await {
            Some(stream) => {
                let async_stream = stream.unwrap();
                let noise_instance = custom_noise::noise::NoiseInstance::responder_from_secret(async_stream, SECRET);
                listen_nn_handshake(noise_instance).await;
                println!("Session established.");
                break ();
            }
            _ => { println!("Some other event") }
        }
    }
} // The stream is closed here

async fn listen_nn_handshake(mut noise_instance: NoiseInstance) {
    noise_instance.handshake_listen().await;
    noise_instance.handshake_send(&[0u8; 0]).await;
    noise_instance.handshake_listen().await;
    noise_instance.transport_listen().await;
}

fn usage_message() {
    println!("
    Usage: ./target/debug/examples/responder [IP] [PORT]
    [IP] and [PORT] are the expected network interfaces, which need to be a valid address for the responder host (example : 127.0.0.1 33100).
    ")
}