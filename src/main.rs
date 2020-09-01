use anyhow::{Context, Result};
use structopt::StructOpt;
use std::net::{ TcpStream};

#[derive(StructOpt)]
struct Cli {
    port: String

}

const IP_ADDRESS: &str =  "127.0.0.1";

fn main() -> Result<()>  {
    let args = Cli::from_args();
    let port: u16 = args.port.parse().with_context(||"could not convert port")?;
    println!("Port {} is open: {}", port, is_port_open(IP_ADDRESS, port)) ;
    Ok(())
}

fn is_port_open(ip_address: &str, port: u16) -> bool {
    match TcpStream::connect((ip_address, port)) {
        Ok(_) => true,
        Err(_) => false
    }
}
