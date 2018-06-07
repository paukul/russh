// #[macro_use] extern crate failure;
extern crate failure;
extern crate byteorder;
extern crate hexdump;
#[macro_use] extern crate log;

use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::str;
use failure::Error;
// use byteorder::{ByteOrder, BigEndian};

mod packet;

use packet::Packet;

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
pub enum MSG_TYPE {
    SSH_MSG_DISCONNECT,
    SSH_MSG_IGNORE,
    SSH_MSG_UNIMPLEMENTED,
    SSH_MSG_DEBUG,
    SSH_MSG_SERVICE_REQUEST,
    SSH_MSG_SERVICE_ACCEPT,
    SSH_MSG_KEXINIT,
    SSH_MSG_NEWKEYS
}

impl From<u8> for MSG_TYPE {
    fn from(n: u8) -> Self {
        use self::MSG_TYPE::*;
        match n {
            1 => SSH_MSG_DISCONNECT,
            2 => SSH_MSG_IGNORE,
            3 => SSH_MSG_UNIMPLEMENTED,
            4 => SSH_MSG_DEBUG,
            5 => SSH_MSG_SERVICE_REQUEST,
            6 => SSH_MSG_SERVICE_ACCEPT,
            20 => SSH_MSG_KEXINIT,
            21 => SSH_MSG_NEWKEYS,
            _ => unreachable!()
        }
    }
}

pub const VERSION: &str = "SSH-2.0-russh_0.1";

pub fn connect(host: &str, port: u16) -> Result<(), Error> {
    let socket_address = format!("{}:{}", host, port);
    debug!("connecting to {}", socket_address);
    let mut stream = TcpStream::connect(socket_address)?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    debug!("Server version: {}", buf.trim_right());
    stream.write_all(format!("{}\r\n", VERSION).as_bytes())?;
    stream.flush()?;

    let mut packet = Packet::read_from(&mut reader)?;
    let msg_type = packet.msg_type();
    debug!("Msg type: {:?}", msg_type);
    packet.discard(16)?;

    debug!("Kex   : {:?}", String::from_utf8(packet.read_str()?)?);
    debug!("SHK   : {:?}", String::from_utf8(packet.read_str()?)?);
    debug!("Enc CS: {:?}", String::from_utf8(packet.read_str()?)?);
    debug!("Enc SC: {:?}", String::from_utf8(packet.read_str()?)?);
    debug!("Mac CS: {:?}", String::from_utf8(packet.read_str()?)?);
    debug!("Mac SC: {:?}", String::from_utf8(packet.read_str()?)?);
    debug!("Com CS: {:?}", String::from_utf8(packet.read_str()?)?);
    debug!("Com SC: {:?}", String::from_utf8(packet.read_str()?)?);
    debug!("Lng CS: {:?}", String::from_utf8(packet.read_str()?)?);
    debug!("Lng SC: {:?}", String::from_utf8(packet.read_str()?)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
