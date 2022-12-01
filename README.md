# custom-noise
Implementation of a noise handshake through raw TCP with snow.

# Noise Test
Rust is needed for compiling the examples:

```
$ cargo build --examples
```
On the responder side:
```
$ ./target/debug/examples/responder [IP] [PORT]
```

On the handshake initiator side:
```
$ ./target/debug/examples/requester [IP] [PORT]
```

On both cases, the IP and PORT are the valid ip and port number that the responder will be listening and bounded to.

The handshake example will finish when one or both nodes disconnect from the specific connection, and will be succesful if the sent message is equivalent to the received message because it will be using the transport created by the snow instance.  The chances on security are given by the payloads of the handshake defined in the listen_nn_handshake and initiate_nn_handshake functions of the "examples/responder.rs" and "examples/requester.rs" provided, and the strength of the original not-shared secrets.

# Usage Example
On the responder side:
```
$ ./target/debug/examples/responder 127.0.0.1 33100
Arguments: ["./target/debug/examples/responder", "127.0.0.1", "33100"]
Listening on address : "127.0.0.1:33100"
Received : we really care
Session established.
$ _
```

On the handshake initiator side:
```
$ ./target/debug/examples/requester 127.0.0.1 33100
Arguments: ["./target/debug/examples/requester", "127.0.0.1", "33100"]
Connecting to address : "127.0.0.1:33100"
Sent : we really care
Session established.
connection closed.
$ _
```