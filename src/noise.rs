use async_std::net::TcpStream;
use async_std::io::{ self, ReadExt, WriteExt };
use snow::TransportState;
use snow::{
    HandshakeState,
    params::NoiseParams, 
    Builder
};

pub struct NoiseInstance {
    pub stream: TcpStream,
    pub buffer: Box<[u8]>,
    pub params: NoiseParams,
    handshake_state: HandshakeState,
}

impl NoiseInstance {
    fn build(stream: TcpStream,params:NoiseParams, handshake_state:HandshakeState) -> Self {
        NoiseInstance {
            stream,
            buffer: Box::new([0u8; 65535]),
            params,
            handshake_state,
        }
    }
    pub fn responder_from_secret(stream: TcpStream, sec: &[u8]) -> Self  {
        let params: NoiseParams = "Noise_XX_25519_ChaChaPoly_SHA256".parse().unwrap();
        let builder: Builder<'_> = Builder::new(params.clone());
        let state = builder.local_private_key(&sec).build_responder().unwrap();
        Self::build(stream, params, state)
    }
    pub fn initiator_from_secret(stream: TcpStream, sec: &[u8]) -> Self {
        let params: NoiseParams = "Noise_XX_25519_ChaChaPoly_SHA256".parse().unwrap();
        let builder: Builder<'_> = Builder::new(params.clone());
        let state = builder.local_private_key(&sec).build_initiator().unwrap();
        Self::build(stream, params, state)
    }

    // .read_message(&recv(&mut stream).unwrap(), &mut buf).unwrap()
    pub async fn handshake_listen(&mut self) -> usize {
        let message = self.recv().await.unwrap();
        self.handshake_state.read_message(&message, &mut self.buffer).unwrap()
    }

    pub async fn handshake_send(&mut self, payload: &[u8]) {
        let len = self.handshake_state.write_message(&payload, &mut self.buffer).unwrap();
        self.send(len).await;
    }

    pub fn into_transport_mode(self) -> Option<TransportState> {
        let transport_state = if self.handshake_state.is_handshake_finished() {
            Some(self.handshake_state.into_transport_mode().unwrap())
        } else { 
            println!("Handshake is not finished yet.");
            None 
        };
        transport_state
    }
    pub async fn transport_listen(mut self) {
        if let Ok(msg) = self.recv().await {
            let mut transport_state = self.handshake_state.into_transport_mode().unwrap();
            let size = transport_state.read_message(&msg, self.buffer.as_mut()).unwrap().clone();
            println!("Received : {}", String::from_utf8_lossy(&self.buffer[..size]));
        };
    }

    pub async fn transport_send(mut self, msg: &[u8]) {
        let mut transport_state = self.handshake_state.into_transport_mode().unwrap();
        let len = transport_state.write_message(msg, &mut self.buffer).unwrap();
        // self.send(len).await;
        let buf = &self.buffer[..len];
        let msg_len_buf = [(buf.len() >> 8) as u8, (buf.len() & 0xff) as u8];
        self.stream.write(&msg_len_buf).await.unwrap();
        self.stream.write(buf).await.unwrap();
    }

    /// Reference : https://github.com/mcginty/snow/blob/master/examples/simple.rs#L110
    pub async fn recv(&mut self) -> io::Result<Vec<u8>> {
        let mut msg_len_buf = [0u8; 2];
        self.stream.read_exact(&mut msg_len_buf).await.unwrap(); 
        let msg_len = ((msg_len_buf[0] as usize) << 8) + (msg_len_buf[1] as usize);
        let mut msg = vec![0u8; msg_len];
        self.stream.read_exact(&mut msg[..]).await.unwrap();
        Ok(msg)
    }

    /// Reference : https://github.com/mcginty/snow/blob/master/examples/simple.rs#L121
    pub async fn send(&mut self, len: usize) {
        let buf = &self.buffer[..len];
        let msg_len_buf = [(buf.len() >> 8) as u8, (buf.len() & 0xff) as u8];
        self.stream.write(&msg_len_buf).await.unwrap();
        self.stream.write(buf).await.unwrap();
    }
}

