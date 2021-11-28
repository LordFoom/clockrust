use std::net::UdpSocket;
use std::{thread,str};
use color_eyre::{eyre::eyre,Report, Result};
use tracing::{ info, error };
use crate::command_runner::CommandRunner;


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

    ///Simple single threaded listener on udp port
    /// Will listen for commands, and interpret correct ones,
    /// returning appropriate success or error responses.
    ///
    /// Commands are:
    /// * clock-in HASH note text
    ///
    /// * clock-out HASH
    ///
    /// Clock in will record start time. One can send multiple clock-in's without sending a clock-out without error.
    ///
    /// Will return "SUCCESS <command HASH>" on success
    /// Will return "FAILURE msg" on failure, which may or may not contain the hash depending on the issude
    pub fn run(&mut self)->Result<()> {
        let server_str = format!("127.0.0.1:{}", self.port);
        let socket = UdpSocket::bind(server_str).expect("Unable to bind to port");
        let mut buffer = [0; 1024];
        loop{
            let socket_new = socket.try_clone().expect("Unable to clone socket");
            let conn = self.connection_string.clone();
            match socket_new.recv_from(&mut buffer){
                Ok((num_bytes, src_addr))=>{
                    thread::spawn(move ||{
                        let send_buffer = &mut buffer[..num_bytes];
                        let received = str::from_utf8(send_buffer).unwrap();
                        let cmd_rnr = CommandRunner::new(conn);
                        info!("Received from client: {}", received);
                        match cmd_rnr.run_command(received){
                           Ok(command) => {
                               info!("Successfully ran: {} ", command);

                           }//todo we need to construct what we're going to return here
                            Err(e) => error!("FAIL: No command was run: {}", e),
                        }
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
