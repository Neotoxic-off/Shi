use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::net::ToSocketAddrs;

use log::{info, error};

pub struct Scan {
    timeout: u64
}

impl Scan {
    pub fn new(timeout: u64) -> Self {
        Self {
            timeout
        }
    }

    pub fn scan(&self, ip: &str, port: u64) -> Result<String, Box<dyn std::error::Error>> {
        let addr: std::net::SocketAddr = format!("{}:{}", ip, port).to_socket_addrs()?.next().unwrap();
        let timeout: Duration = Duration::from_secs(self.timeout);
        let stream: TcpStream = TcpStream::connect_timeout(&addr, timeout)?;
        let mut reader: BufReader<TcpStream> = BufReader::new(stream);
        let mut banner: String = String::new();

        reader.read_line(&mut banner)?;

        Ok(banner.trim().to_string())
    }
}
