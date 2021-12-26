use std::{str, thread};
use std::net::UdpSocket;
use std::sync::Arc;

use color_eyre::{Result};
use tracing::{error, info};

use crate::command::{create_command};
use crate::db::ClockRuster;

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
    pub fn run(self)->Result<()> {
        let server_str = format!("127.0.0.1:{}", self.port);
        let socket = UdpSocket::bind(server_str).expect("Unable to bind to port");
        let mut buffer = [0; 1024];
        // let conn = Arc::new(self.connection_string.clone());
        //scoped threads from crossbeam? cos this move is killing me
        // let conn = self.connection_string.clone();
        loop{
            let socket_new = socket.try_clone().expect("Unable to clone socket");
            let tmp = self.connection_string.clone();
            match socket_new.recv_from(&mut buffer){
                Ok((num_bytes, src_addr))=>{
                    thread::spawn(move ||{
                        let cr = ClockRuster::init(&tmp);
                        let send_buffer = &mut buffer[..num_bytes];
                        let received = str::from_utf8(send_buffer).unwrap();
                        // let cmd_rnr = CommandConstructor::new(conn, &mut ClockRust::default());
                        info!("Received from client: {}", received);
                         match create_command(received){
                           Ok(cmd) => {
                               // let cmd: dyn Command = *box_cmd;
                               // info!("Successfully created: {} ", *box_cmd.to_string());
                               match cr.run_clock_command(cmd){
                                   Ok(_) => { //TODO write back success
                                        }
                                   Err(_) => { //TODO write back failure
                                        }
                               }
                           }
                            Err(e) => {
                                error!("FAIL: No command returned: {}", e)
                                //TODO write back failure
                            },
                        };
                        //here we need to store the command in the sqlite table
                        let response_string = format!("Received this: {}", String::from_utf8_lossy(send_buffer));
                        socket_new.send_to(response_string.as_bytes(), &src_addr).expect("Error sending datagram to remote socket");

                    });
                }
                Err(err) => {
                    println!("Error in receiving datagram over UDP: {}", err);
                }
            }

        }
    }


}
