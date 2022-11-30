use futures::TryFutureExt;
// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener};
// use libp2p_core::{
//     address_translation,
//     multiaddr::{Multiaddr, Protocol},
//     transport::{ListenerId, TransportError, TransportEvent},
// };
use async_std::net::TcpStream;
use async_std::io::{ self, ReadExt, WriteExt };
use snow::{
    HandshakeState,
    params::NoiseParams, 
    Builder
};

struct NoiseInstance {
    stream: TcpStream,
    s: Vec<u8>,
    secret_phrase: Box<[u8]>,
    params: NoiseParams,
    state: HandshakeState
}

impl NoiseInstance {
    fn build(&mut self, stream: TcpStream, boxed_sec: Box<[u8;128]>) -> Self {
        let builder: Builder<'_> = Builder::new(self.params.clone());
        NoiseInstance {
            stream,
            s : builder.generate_keypair().unwrap().private,
            secret_phrase: boxed_sec,
            params: "Noise_XXpsk3_25519_ChaChaPoly_BLAKE2s".parse().unwrap(),
            state : builder.local_private_key(&self.s).psk(3, &self.secret_phrase).build_responder().unwrap()
        }
    }
    pub fn from_secret(&mut self, stream: TcpStream, sec: [u8;128]) -> Self  {
        self.build(stream, Box::new(sec))
    }

    /// Hyper-basic stream transport receiver. 16-bit BE size followed by payload.
    fn recv(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
        let mut msg_len_buf = [0u8; 2];
        stream.read_exact(&mut msg_len_buf).unwrap_or_else(|_| {});
        let msg_len = ((msg_len_buf[0] as usize) << 8) + (msg_len_buf[1] as usize);
        let mut msg = vec![0u8; msg_len];
        stream.read_exact(&mut msg[..]).unwrap_or_else(|_| {});
        Ok(msg)
    }

    /// Hyper-basic stream transport sender. 16-bit BE size followed by payload.
    fn send(stream: &mut TcpStream, buf: &[u8]) {
        let msg_len_buf = [(buf.len() >> 8) as u8, (buf.len() & 0xff) as u8];
        stream.write_all(&msg_len_buf).unwrap_or_else(|_| {});
        stream.write_all(buf).unwrap_or_else(|_| {});
    }
}

