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

    debug!("Kex algos: {:?}", String::from_utf8(packet.read_str()?)?);
    // let (_, mut buf) = packet.payload.split_at(17);
    // let mut algorithms = consume_string(&mut buf);
    // debug!("Kex algorithms: {}", String::from_utf8(algorithms.to_vec())?);
    // algorithms = consume_string(&mut buf);
    // debug!("server_host_key_algorithms: {}", String::from_utf8(algorithms.to_vec())?);
    // algorithms = consume_string(&mut buf);
    // debug!("encryption_algorithms_client_to_server: {}", String::from_utf8(algorithms.to_vec())?);
    // algorithms = consume_string(&mut buf);
    // debug!("encryption_algorithms_server_to_client: {}", String::from_utf8(algorithms.to_vec())?);
    // algorithms = consume_string(&mut buf);
    // debug!("mac_algorithms_client_to_server: {}", String::from_utf8(algorithms.to_vec())?);
    // algorithms = consume_string(&mut buf);
    // debug!("mac_algorithms_server_to_client: {}", String::from_utf8(algorithms.to_vec())?);
    // algorithms = consume_string(&mut buf);
    // debug!("compression_algorithms_client_to_server: {}", String::from_utf8(algorithms.to_vec())?);
    // algorithms = consume_string(&mut buf);
    // debug!("compression_algorithms_server_to_client: {}", String::from_utf8(algorithms.to_vec())?);
    // algorithms = consume_string(&mut buf);
    // debug!("languages_client_to_server: {}", String::from_utf8(algorithms.to_vec())?);
    // algorithms = consume_string(&mut buf);
    // debug!("languages_server_to_client: {}", String::from_utf8(algorithms.to_vec())?);

    // let (_, tail) = packet.payload.split_at(17);
    // let str_length = BigEndian::read_u32(&tail[0..4]) as usize;
    // trace!("Kex algorithm string length: {}", str_length);
    // let kex_algorithms = &tail[4..str_length + 4];
    // debug!("Kex algorithms: {}", String::from_utf8(kex_algorithms.to_vec())?);

    // let (_, tail) = tail.split_at(str_length + 4);
    // let str_length = BigEndian::read_u32(&tail[0..4]) as usize;
    // trace!("Server host key algorithm string length: {}", str_length);
    // let server_host_key_algorithms = &tail[4..str_length + 4];
    // debug!("Server host key algorithms: {}", String::from_utf8(server_host_key_algorithms.to_vec())?);

    // let (_, tail) = tail.split_at(str_length + 4);
    // let str_length = BigEndian::read_u32(&tail[0..4]) as usize;
    // trace!("Enccs string length: {}", str_length);
    // let enc_client_server = &tail[4..str_length + 4];
    // debug!("Enccs algorithms: {}", String::from_utf8(enc_client_server.to_vec())?);

    Ok(())
}

// fn consume_string<'a>(buf: &mut &'a[u8]) -> &'a[u8] {
//     let str_length = BigEndian::read_u32(buf) as usize;
//     trace!("String length: {}", str_length);
//     let name_list = &buf[4..str_length + 4];
    
//     *buf = &buf[str_length + 4..];
//     name_list
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
