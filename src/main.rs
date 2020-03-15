use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::{BufReader, BufWriter, Result};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::path::Path;

struct Sender;

impl Sender {
    fn new() -> Self {
        Sender
    }

    fn send<A: ToSocketAddrs, P: AsRef<Path>>(&self, path: P, addr: A) -> Result<()> {
        file_to_stream(path, TcpStream::connect(addr)?)?;
        Ok(())
    }
}

struct Receiver {
    listener: TcpListener,
}

impl Receiver {
    fn new<A: ToSocketAddrs>(addr: A) -> Self {
        Receiver {
            listener: TcpListener::bind(addr).unwrap(),
        }
    }

    fn receive<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        println!("Receiver: {:?}", self.listener.local_addr().unwrap());
        match self.listener.accept() {
            Ok((stream, addr)) => {
                println!("Sender:   {:?}", addr);
                stream_to_file(stream, path)?;
            }
            Err(e) => {
                println!("Error:    {:?}", e);
            }
        }
        Ok(())
    }
}

fn file_to_stream<P: AsRef<Path>>(path: P, stream: TcpStream) -> Result<u64> {
    std::io::copy(
        &mut BufReader::new(File::open(path)?),
        &mut BufWriter::new(stream),
    )
}

fn stream_to_file<P: AsRef<Path>>(stream: TcpStream, path: P) -> Result<u64> {
    std::io::copy(
        &mut BufReader::new(stream),
        &mut BufWriter::new(File::create(path)?),
    )
}

fn main() -> Result<()> {
    let matches = App::new("Simple TCP file sender/receiver")
        .version("0.1.0")
        .author("E-Neo <e-neo@qq.com>")
        .about("As simple as possible")
        .subcommand(
            SubCommand::with_name("send")
                .arg(Arg::with_name("PATH").required(true))
                .arg(Arg::with_name("ADDR").required(true)),
        )
        .subcommand(
            SubCommand::with_name("receive")
                .arg(Arg::with_name("PATH").required(true))
                .arg(
                    Arg::with_name("bind")
                        .value_name("ADDR")
                        .default_value("0.0.0.0:8000"),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("send") {
        Sender::new().send(
            matches.value_of("PATH").unwrap(),
            matches.value_of("ADDR").unwrap(),
        )?;
    } else if let Some(matches) = matches.subcommand_matches("receive") {
        Receiver::new(matches.value_of("bind").unwrap())
            .receive(matches.value_of("PATH").unwrap())?;
    } else {
        println!("{}", matches.usage());
    }
    Ok(())
}
