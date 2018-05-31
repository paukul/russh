use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::str;

pub const VERSION: &str = "SSH-2.0-russh_0.1";
const MAX_PACKET_SIZE: usize = 35000;

pub fn connect(host: &str, port: u16) {
    let socket_address = format!("{}:{}", host, port);
    println!("connecting to {}", socket_address);
    let mut stream = TcpStream::connect(socket_address).unwrap();
    let mut buf = String::new();

    let mut raw_reader = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    reader.read_line(&mut buf).unwrap();
    println!("Server version: {}", buf);

    stream.write(format!("{}\r\n", VERSION).as_bytes()).unwrap();
    stream.flush().unwrap();
    reader.read_line(&mut buf).unwrap();
    println!("Server version: {}", buf);
    let mut buf = [0; MAX_PACKET_SIZE];
    let bytes_read = raw_reader.read(&mut buf).unwrap();
    println!("received {} bytes", bytes_read);
    println!("{}", str::from_utf8(&buf).unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
