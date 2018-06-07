use std::io::{self, Read};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;
use super::MSG_TYPE;

// const MAX_PACKET_SIZE: usize = 35000;

pub struct Packet {
  raw: Vec<u8>,
  #[allow(dead_code)]
  padding_length: u8,
  #[allow(dead_code)]
  payload_length: usize,
  pos: usize
}

impl Packet {
  pub fn read_from<R: io::Read>(stream: &mut R) -> Result<Packet, Error> {
    let packet_length = stream.read_u32::<BigEndian>()? as usize;
    let mut raw = Vec::with_capacity(packet_length);
    raw.write_u32::<BigEndian>(packet_length as u32)?;
    let read = stream.take(packet_length as u64).read_to_end(&mut raw)?;
    let padding_length = raw[4];
    let payload_length = packet_length - usize::from(padding_length) - 1;
    if read < packet_length {
      Err(io::Error::new(io::ErrorKind::BrokenPipe, "broken stream").into())
    } else {
      Ok(Packet{
          raw,
          padding_length,
          payload_length,
          pos: 6
        }
      )
    }
  }

  pub fn msg_type(&self) -> MSG_TYPE {
    trace!("Msg Type: {}", self.raw[5]);
    MSG_TYPE::from(self.raw[5])
  }

  pub fn discard(&mut self, len: usize) -> Result<(), Error> {
    let mut buf = Vec::new();
    self.take(len as u64).read_to_end(&mut buf)?;
    Ok(())
  }

  pub fn read_str(&mut self) -> io::Result<Vec<u8>> {
    let str_len = self.read_u32::<BigEndian>()? as usize;
    let mut buf = Vec::with_capacity(str_len);
    self.take(str_len as u64).read_to_end(&mut buf)?;
    Ok(buf)
  }
}

impl Read for Packet {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    let mut reader = &self.raw[self.pos..];
    let read = reader.read(buf)?;
    self.pos = self.pos + read;
    Ok(read)
  }
}