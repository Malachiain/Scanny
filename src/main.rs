use anyhow::{ Result};
use structopt::StructOpt;
use std::net::{ TcpStream};
use std::path::PathBuf;
use std::ops::Range;
use std::fs::File;
use std::io::Write;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "f", long = "from", help = "the port to start scanning from", default_value = "1")]
    from: u16 ,
    #[structopt(short = "t", long = "to", help = "the port to finish scanning at", default_value = "65535")]
    to: u16 ,
    #[structopt(help = "Output file, stdout if not present", parse(from_os_str))]
    output: Option<PathBuf>
}

const IP_ADDRESS: &str =  "127.0.0.1";

fn main() -> Result<()>  {
    let args = Cli::from_args();
    let scan_range = Range { start: args.from, end: args.to };
    match args.output {
        Some(file_path) => scan_to_outfile(file_path, scan_range),
        None => scan_to_sysout(scan_range)
    }
    Ok(())
}

fn scan_to_sysout(scan_range: Range<u16>){
    println!("{:10}|{:5}|{}", "port", "open", "service");
    println!("{:-<25}","");
    for p in scan_range {
        let is_open = is_port_open(IP_ADDRESS, p);
        if is_open {
            println!("{:<10}|{:5}|{}", p, is_open, "NA");
        }
    }
} 

fn scan_to_outfile(outpath: PathBuf, scan_range: Range<u16>) {
    let mut out_string = String::from("port,open\n");
    for p in scan_range {
        let is_open = is_port_open(IP_ADDRESS, p);
        if is_open {
            out_string.push_str(&format!("{},{}\n", p, is_open));
        }
    }
    match outpath.to_str() {
        Some(s) => {  
        println!("file path {}", s);
        println!("output {}", out_string);
        let mut f = File::create(s).expect("Unable to create file");
        f.write_all(out_string.as_bytes()).expect("Unable to write data");
        },
        None => println!("couldn't convert to sting")
    }
   
}



fn is_port_open(ip_address: &str, port: u16) -> bool {
    match TcpStream::connect((ip_address, port)) {
        Ok(_) => true,
        Err(_) => false
    }
}
