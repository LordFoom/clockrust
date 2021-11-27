use clap::{App, ArgMatches};
use color_eyre::Report;
use tracing_subscriber::EnvFilter;

pub fn setup(verbose: bool) ->Result<(), Report>{
    if std::env::var("RUST_LIB_BACKTRACE").is_err(){
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if verbose {
        std::env::set_var("RUST_LOG","info")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}

///Get our cli arguments and return them in a nice data structure
/// Current args:
/// verbose: log stuff
/// port: listen here
/// file: sqlite db file
pub fn parse_args()->ArgMatches{
    App::new("clockrust")
        .version("0.1")
        .author("foom")
        .about("Time tracking server and app")
        .arg("-v, --verbose 'Log much information' ")
        .arg("-p, --port 'Port number'")
        .arg("-f, --file 'SQLite file where we store times'")
        .get_matches()
}
