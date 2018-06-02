// #[macro_use] extern crate failure;
extern crate failure;
extern crate byteorder;
#[macro_use] extern crate log;

use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::str;
use failure::Error;
use byteorder::{ByteOrder, BigEndian};

// SSH_MSG_DISCONNECT             1
// SSH_MSG_IGNORE                 2
// SSH_MSG_UNIMPLEMENTED          3
// SSH_MSG_DEBUG                  4
// SSH_MSG_SERVICE_REQUEST        5
// SSH_MSG_SERVICE_ACCEPT         6
// SSH_MSG_KEXINIT                20
// SSH_MSG_NEWKEYS                21

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum MSG_TYPE {
    SSH_MSG_DISCONNECT,
    SSH_MSG_IGNORE,
    SSH_MSG_UNIMPLEMENTED,
    SSH_MSG_DEBUG,
    SSH_MSG_SERVICE_REQUEST,
    SSH_MSG_SERVICE_ACCEPT,
    SSH_MSG_KEXINIT,
    SSH_MSG_NEWKEYS
}

#[allow(dead_code)]
struct Packet {
    packet_length: u32,
    payload: Vec<u8>,
    padding: Vec<u8>,
    mac: Vec<u8>
}

impl Packet {
    fn new(buf: &[u8]) -> Packet {
        let packet_length = BigEndian::read_u32(&buf[0..4]);
        let padding_length = *&buf[4] as u32;
        let payload_length = packet_length - padding_length - 1;
        let last_payload_byte = payload_length - padding_length - 1;
        let payload = &buf[5..last_payload_byte as usize];
        trace!("Packet Length: {}", packet_length);
        trace!("Padding Length: {}", padding_length);
        Packet {
            packet_length: packet_length,
            payload: payload.to_vec(),
            padding: Vec::new(),
            mac: Vec::new()
        }
    }
}

impl From<u8> for MSG_TYPE {
    fn from(n: u8) -> Self {
        match n {
            1 => MSG_TYPE::SSH_MSG_DISCONNECT,
            2 => MSG_TYPE::SSH_MSG_IGNORE,
            3 => MSG_TYPE::SSH_MSG_UNIMPLEMENTED,
            4 => MSG_TYPE::SSH_MSG_DEBUG,
            5 => MSG_TYPE::SSH_MSG_SERVICE_REQUEST,
            6 => MSG_TYPE::SSH_MSG_SERVICE_ACCEPT,
            20 => MSG_TYPE::SSH_MSG_KEXINIT,
            21 => MSG_TYPE::SSH_MSG_NEWKEYS,
            _ => unreachable!()
        }
    }
}

pub const VERSION: &str = "SSH-2.0-russh_0.1";
const MAX_PACKET_SIZE: usize = 35000;

pub fn connect(host: &str, port: u16) -> Result<(), Error> {
    let socket_address = format!("{}:{}", host, port);
    debug!("connecting to {}", socket_address);
    let mut stream = TcpStream::connect(socket_address)?;
    let mut buf = String::new();

    let mut raw_reader = stream.try_clone()?;
    let mut reader = BufReader::new(stream.try_clone()?);
    reader.read_line(&mut buf)?;
    debug!("Server version: {}", buf.trim_right());

    stream.write(format!("{}\r\n", VERSION).as_bytes())?;
    stream.flush()?;

    let mut buf = [0; MAX_PACKET_SIZE];
    raw_reader.read(&mut buf)?;
    let packet = Packet::new(&buf);
    let msg_type = packet.payload[0];
    debug!("Msg type: {:?}", MSG_TYPE::from(msg_type));
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
