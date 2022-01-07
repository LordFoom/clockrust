use std::{str, thread};
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;

use color_eyre::Result;
use tracing::{error, info};

use crate::command::create_command;
use crate::db::ClockRuster;

pub struct ClockRustServer {
    ///port we will listen  on
    port: u16,
    ///db connection string
    connection_string: String,
}

impl ClockRustServer {
    pub fn new(port: u16, connection_string: String) -> ClockRustServer {
        ClockRustServer {
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
    pub fn run(& self) -> Result<()> {
        let server_str = format!("127.0.0.1:{}", self.port);
        let socket = UdpSocket::bind(server_str).expect("Unable to bind to port");
        let mut buffer = [0; 1024];
        let tmp_pre = self.connection_string.clone();
        loop {
            let socket_new = socket.try_clone().expect("Unable to clone socket");
            let tmp = tmp_pre.clone();
            match socket_new.recv_from(&mut buffer) {
                Ok((num_bytes, src_addr)) => {
                    thread::spawn(move || {
                        let cr = ClockRuster::init(&tmp);
                        let send_buffer = &mut buffer[..num_bytes];
                        let received = str::from_utf8(send_buffer).unwrap();
                        info!("Received from client: {}", received);
                        match create_command(received) {
                            Ok(cmd) => {
                                let cmd_type = cmd.command.clone();
                                let task = cmd.task.clone();
                                match cr.run_clock_command(cmd) {
                                    Ok(_) => {
                                        let response_msg = format!("Successfully ran {} for: {}", cmd_type, task);
                                        let response_buffer = response_msg.as_bytes();
                                        socket_new.send_to(response_buffer, src_addr).expect("Error sending success message");
                                    }
                                    Err(why) => {
                                        let response_msg = format!("FAILED to run {} for: {}, because: {}", cmd_type, task, why);
                                        let response_buffer = response_msg.as_bytes();
                                        socket_new.send_to(response_buffer, src_addr).expect("Error sending failure message");
                                        // &self.write_to_socket(&socket_new, &src_addr, &response_msg);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("FAIL: No command returned: {}", e)
                                //TODO write back failure
                            }
                        };
                        // let response_string = format!("Received this: {}", String::from_utf8_lossy(send_buffer));
                        // socket_new.send_to(response_string.as_bytes(), &src_addr).expect("Error sending datagram to remote socket");
                    });
                }
                Err(err) => {
                    println!("Error in receiving datagram over UDP: {}", err);
                }
            }
        }
    }

    ///On this socket, to that source address, send this message
    fn write_to_socket(&self, socket:&UdpSocket, src_addr:&SocketAddr,msg:&str){
        let response_buffer = msg.as_bytes();
        socket.send_to(response_buffer, src_addr).expect("Error sending failure message");
    }
}
