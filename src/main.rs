use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::{BufReader, BufWriter, Result};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::path::Path;

struct Server {
    listener: TcpListener,
}

impl Server {
    fn new<A: ToSocketAddrs>(addr: A) -> Self {
        Server {
            listener: TcpListener::bind(addr).unwrap(),
        }
    }

    fn recv<P: AsRef<Path>>(&self, path: P) -> Result<u64> {
        println!("Receiver: {:?}", self.listener.local_addr().unwrap());
        match self.listener.accept() {
            Ok((stream, addr)) => {
                println!("Client:   {:?}", addr);
                stream_to_file(stream, path)
            }
            Err(e) => {
                println!("Error:    {:?}", e);
                Err(e)
            }
        }
    }

    fn send<P: AsRef<Path>>(&self, path: P) -> Result<u64> {
        println!("Sender: {:?}", self.listener.local_addr().unwrap());
        match self.listener.accept() {
            Ok((stream, addr)) => {
                println!("Client: {:?}", addr);
                file_to_stream(path, stream)
            }
            Err(e) => {
                println!("Error:  {:?}", e);
                Err(e)
            }
        }
    }
}

struct Client;

impl Client {
    fn new() -> Self {
        Client
    }

    fn get<A: ToSocketAddrs, P: AsRef<Path>>(&self, addr: A, path: P) -> Result<u64> {
        stream_to_file(TcpStream::connect(addr)?, path)
    }

    fn post<A: ToSocketAddrs, P: AsRef<Path>>(&self, addr: A, path: P) -> Result<u64> {
        file_to_stream(path, TcpStream::connect(addr)?)
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
    let matches = App::new("Simple TCP file-sharing tool")
        .version("0.1.0")
        .author("E-Neo <e-neo@qq.com>")
        .subcommand(
            SubCommand::with_name("send")
                .about("Server to send a file")
                .arg(Arg::with_name("PATH").required(true))
                .arg(Arg::with_name("ADDR").default_value("0.0.0.0:8000")),
        )
        .subcommand(
            SubCommand::with_name("recv")
                .about("Server to receive a file")
                .arg(Arg::with_name("PATH").required(true))
                .arg(Arg::with_name("ADDR").default_value("0.0.0.0:8000")),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Client to get a file")
                .arg(Arg::with_name("ADDR").required(true))
                .arg(Arg::with_name("PATH").required(true)),
        )
        .subcommand(
            SubCommand::with_name("post")
                .about("Client to post a file")
                .arg(Arg::with_name("ADDR").required(true))
                .arg(Arg::with_name("PATH").required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("send") {
        Server::new(matches.value_of("ADDR").unwrap()).send(matches.value_of("PATH").unwrap())?;
    } else if let Some(matches) = matches.subcommand_matches("recv") {
        Server::new(matches.value_of("ADDR").unwrap()).recv(matches.value_of("PATH").unwrap())?;
    } else if let Some(matches) = matches.subcommand_matches("get") {
        Client::new().get(
            matches.value_of("ADDR").unwrap(),
            matches.value_of("PATH").unwrap(),
        )?;
    } else if let Some(matches) = matches.subcommand_matches("post") {
        Client::new().post(
            matches.value_of("ADDR").unwrap(),
            matches.value_of("PATH").unwrap(),
        )?;
    } else {
        println!("{}", matches.usage());
    }
    Ok(())
}
