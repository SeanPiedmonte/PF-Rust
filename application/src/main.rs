use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const MSG_SIZE: usize = 32;

fn main() -> std::io::Result<()> {
    let mut client = TcpStream::connect("127.0.0.1:7878").expect("Could not connect to port");
    client.set_nonblocking(true).expect("failed to initiate nonblocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message recv {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                    println!("connection with server was severed");
                    break;
            }
        }
    });

    let out_msg = "Im want some Finance Data";
    let out_bytes = out_msg.as_bytes();
    client.write(&out_bytes)?;
    

    Ok(())
}

