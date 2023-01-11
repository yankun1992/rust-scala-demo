use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};

use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream};

// Some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);

fn main() -> Result<(), Box<dyn Error>> {
    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the server socket.
    let addr = "127.0.0.1:13265".parse()?;
    let mut server = TcpListener::bind(addr)?;
    println!("server: {:?}", server);
    // Start listening for incoming connections.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    println!("{:?}", poll.registry());

    let mut id: usize = 1;
    let mut client_map: HashMap<usize, TcpStream> = HashMap::with_capacity(10);

    let resp = "HTTP/1.1 200 OK
Connection: keep-alive
Server: ATTORE-HTTPD/1.1
Content-Length: 11

hello world";

    // Start an event loop.
    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;
        println!("poll success");
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let (client_stream, client_address) = server.accept()?;
                    if let Some(stream) = client_map.insert(id, client_stream) {
                        panic!("Stream entry token failed!");
                    }
                    poll.registry().register(client_map.get_mut(&id).unwrap(), Token(id), Interest::READABLE)?;
                    println!("accept connect from {}", client_address);
                    id += 1;
                }
                Token(id) => {
                    if event.is_readable() {
                        let client = client_map.get_mut(&id).unwrap();
                        let buf = &mut [0u8; 1024];
                        match client.read(buf) {
                            Ok(read) => {
                                println!("Received {} bytes", read);
                                println!("{:?}", String::from_utf8_lossy(&buf[0..read]));
                                client.write(resp.as_bytes()).expect("TODO: panic message ");
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}