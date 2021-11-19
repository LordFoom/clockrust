use std::error::Error;
use std::str;
use std::thread;
use std::net::UdpSocket;
use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use clap::{Arg, App, ArgMatches};

fn setup()->Result<(), Report>{
    if std::env::var("RUST_LIB_BACKTRACE").is_err(){
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG","info")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}

///Get our cli arguments and return them in a nice data structure
fn parse_args()->ArgMatches{
    App::new("clockrust")
        .version("0.1")
        .author("foom")
        .about("Time tracking server and app")
        .arg("-v, --verbose 'Log much information' ")
        .arg("-p, --port 'Port number'")
        .get_matches()
}


fn main()->Result<(), Report> {
    setup()?;
    //need to put this in its own file and then have a parser maybe
    let socket = UdpSocket::bind("127.0.0.1:4420").expect("Unable to bind to port");
    let mut buffer = [0; 1024];
    info!("Listening....");

    loop{
        let socket_new = socket.try_clone().expect("Unable to clone socket");
        match socket_new.recv_from(&mut buffer){
            Ok((num_bytes, src_addr))=>{
                thread::spawn(move ||{
                    let send_buffer = &mut buffer[..num_bytes];
                    println!("Received from client: {}", str::from_utf8(send_buffer).unwrap());
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
