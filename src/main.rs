mod server;
mod config;
mod command_runner;


use std::error::Error;
use std::str;
use std::thread;
use std::net::UdpSocket;
use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use clap::{Arg, App, ArgMatches};
use crate::server::ClockRustServer;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
// use hyper::{Body, Request, Response, Server};


///Configure and start our timetracker
fn main()->Result<(), Report> {
    let args  = config::parse_args();
    let verbose = args.is_present("verbose");
    config::setup(verbose)?;
    let port = u16::from_str( args.value_of("port")
        .unwrap_or("4420"))
        .unwrap();//want it to complain for bad u16 values

    let connection_string = args.value_of("file").unwrap_or("~/.clockrust.db");
    //need to put this in its own file and then have a parser maybe
    info!("Listening....");
    let mut crs = ClockRustServer::new(port, connection_string.to_string() );
    crs.run()?;

    Ok(())
}
