//836a279aff62841476697ac8d7f2cd973c2a101eda36daee9f577fc6b35fafcb  port-mac.tar.gz
use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX : u16 = 65535;

struct Arguments{
    flag : String,
    ipaddr : IpAddr,
    threads : u16
}

fn help_message(){
    println!("Port sniffer CLI\nUSAGE : port [OPTIONS] [PORT]\nOPTIONS:\n-h | --help : Display this message\n-t | --threads : Specify the number of threads(Default : 4)\n\nCreated by Japroz Singh Saini\nLicense : MIT");
}

fn scan(tx : Sender<u16>, start_port : u16, addr : IpAddr, num_threads : u16){
    let mut port : u16 = start_port + 1;
    loop{
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(" . ");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

impl Arguments{
    fn new(args : &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2{
            help_message();
            return Err("help");
        }else if args.len() > 4{
            return Err("Too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments {flag : String::from(""), ipaddr, threads : 4});
        }else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2{
                help_message();
                return Err("help");
            }else if flag.contains("-h") || flag.contains("--help") {
                return Err("Too many arguments");
            }else if flag.contains("-t") || flag.contains("--threads") {
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IP Address; IP Address must be IPv4 or IPv6")
                };

                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            }else {
                return Err("Arguments were provided in an invalid syntax. See `port -h` for further help")
            }
        }
    }
}

fn main(){
    let args : Vec<String> = env::args().collect();
    let _program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                process::exit(0);
            }else{
                eprintln!("Port : Problem parsing the arguments : {}", err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads{
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr,num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);   // This drops tx out of the scope
    for p in rx{
        out.push(p)
    }
    println!("---------------------");
    out.sort();
    for v in out{
        print!("Port {}:{} is open", addr, v)
    }
}