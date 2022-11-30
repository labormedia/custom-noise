use snow;

fn main() {
    let mut noise = snow::Builder::new("Noise_NN_25519_ChaChaPoly_BLAKE2s".parse().unwrap())
    .build_initiator().unwrap();

    let mut buf = [0u8; 65535];

    // write first handshake message
    let first = noise.write_message(&[], &mut buf).unwrap();

    // receive response message
    let incoming = "Hello Noise".to_string().into_bytes();//receive_message_from_the_mysterious_ether();
    noise.read_message(&incoming, &mut buf).unwrap();

    // complete handshake, and transition the state machine into transport mode
    let mut noise = noise.into_transport_mode().unwrap();
}
