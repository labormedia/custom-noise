use async_std::net::{TcpListener};
use custom_noise::noise::NoiseInstance;
use async_std::stream::StreamExt;

#[async_std::main]
async fn main() {
    static SECRET: &[u8] = b"we care a lot";
    // secret.copy_from_slice(b"we care a lot") ;
    let address = "127.0.0.1:33100";
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
