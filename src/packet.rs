use std::io::{self, Read};
use byteorder::{BigEndian, ReadBytesExt};
use failure::Error;

pub struct Packet {
  payload: Vec<u8>
}

impl Packet {
  fn read_from<R: io::Read>(stream: &mut R) -> Result<Packet, Error> {
    let payload_length = stream.read_u32::<BigEndian>()? as usize;
    let mut payload = Vec::with_capacity(payload_length);
    let read = stream.take(payload_length as u64).read_to_end(&mut payload);
    Ok(Packet{payload})
  }
}