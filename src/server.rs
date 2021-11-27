use std::error::Error;
use std::net::UdpSocket;
use std::{thread,str};
use color_eyre::Report;
use tracing::info;


pub struct ClockRustServer{
    ///port we will listen  on
    port: u16,
    ///db connection string
    connection_string:String,
}

impl ClockRustServer{

    pub fn new(port:u16, connection_string: String)->ClockRustServer{
        ClockRustServer{
            port,
            connection_string,
        }
    }

    pub fn run(&mut self)->Result<(), Report>{
        let server_str = format!("127.0.0.1:{}", self.port);
        let socket = UdpSocket::bind(server_str).expect("Unable to bind to port");
        let mut buffer = [0; 1024];
        loop{
            let socket_new = socket.try_clone().expect("Unable to clone socket");
            match socket_new.recv_from(&mut buffer){
                Ok((num_bytes, src_addr))=>{
                    thread::spawn(move ||{
                        let send_buffer = &mut buffer[..num_bytes];
                        info!("Received from client: {}", str::from_utf8(send_buffer).unwrap());
                        //here we need to store the command in the sqlite table
                        let response_string = format!("Received this: {}", String::from_utf8_lossy(send_buffer));
                        socket_new.send_to(&response_string.as_bytes(), &src_addr).expect("Error sending datagram to remote socket");

                    });
                }
                Err(err) => {
                    println!("Error in receiving datagram over UDP: {}", err);
                }
            }

        }
    }

}
