use std::{
    io::{prelude::*, BufReader, BufWriter},
    net::{TcpListener, TcpStream, Shutdown},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:33100").unwrap();
    loop {
        match listener.incoming().next() {
            Some(stream) => {
                let stream = stream.unwrap();
        
                handle(stream);
                break ();
            }
            _ => { println!("Some other event") }
        }
    }
}

fn handle(mut stream: TcpStream) {

}

fn default_handle(mut stream: TcpStream) {
    println!("handling connection");
    let buf_reader = BufReader::new(&mut stream);
    let tcp_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| {
            println!("got here");
            !line.contains("some_handle")
        })
        .collect();
    println!("Request: {:#?}", tcp_request);

    println!("Hello response");
    stream.write(&"Thank you\nH\nsome_handle".to_string().into_bytes()).unwrap();
}
