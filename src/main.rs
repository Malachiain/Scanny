use anyhow::{ Result};
use structopt::StructOpt;
use std::net::{ TcpStream};

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "f", long = "from", help = "the port to start scanning from", default_value = "1")]
    from: u16 ,
    #[structopt(short = "t", long = "to", help = "the port to finish scanning at", default_value = "65535")]
    to: u16 ,

}

const IP_ADDRESS: &str =  "127.0.0.1";

fn main() -> Result<()>  {
    let args = Cli::from_args();
    let scan_range = std::ops::Range { start: args.from, end: args.to };
    println!("{:10}|{:5}|{}", "port", "open", "service");
    println!("{:-<25}","");

    for p in scan_range {
        let is_open = is_port_open(IP_ADDRESS, p);
        if is_open {
            println!("{:<10}|{:5}|{}", p, is_open, "NA");
        }
    }
    
   
    Ok(())
}

fn is_port_open(ip_address: &str, port: u16) -> bool {
    match TcpStream::connect((ip_address, port)) {
        Ok(_) => true,
        Err(_) => false
    }
}
