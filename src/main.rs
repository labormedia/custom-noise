use async_std::net::TcpListener;
use custom_noise;
use zeroize::{
    Zeroize,
    Zeroizing
};

#[async_std::main]
async fn main() {
    let mut secret = b"we care a lot";
    let ( async_listener, _) = TcpListener::bind("127.0.0.1:33100").await.unwrap().accept().await.unwrap();
    let mut noise_instance = custom_noise::noise::NoiseInstance::responder_from_secret(async_listener, secret);

}
