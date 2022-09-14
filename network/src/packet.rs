
// ===== Imports =====
use uuid::{Uuid};
use bytes::{Buf, BufMut, BytesMut};
// ===================

pub(crate) const HEADER_SIZE: usize = 17;

#[derive(Clone)]
pub(crate) struct Packet {
  pub(crate) header: Header,
  pub payload: BytesMut,
}

impl Packet {
  pub fn to_bytes(self) -> BytesMut {
    let mut byts = BytesMut::new();
    byts.put(self.payload);
    byts
  }

  pub fn from_bytes(byts: BytesMut) -> Self {
    let header_byts = BytesMut::from(&byts[..=HEADER_SIZE]);
    let header = Header::from_bytes(header_byts);
    let payload = BytesMut::from(&byts[(HEADER_SIZE+1)..]);
    Self { header, payload }
  }
}

#[derive(Clone)]
pub(crate) struct Header {
  pub(crate) packet_type: PacketType,
  pub(crate) from: Uuid,
}

impl Header {
  pub fn to_bytes(self) -> BytesMut {
    let mut byts = BytesMut::new();
    byts.put_u8(self.packet_type as u8);
    byts.put_u128(self.from.as_u128());
    byts
  }

  pub fn from_bytes(mut byts: BytesMut) -> Self {
    let packet_type = PacketType::from(byts.get_u8());
    let from = Uuid::from_u128(byts.get_u128());
    Self { packet_type, from }
  }
}

#[repr(u8)]
#[derive(Clone)]
pub(crate) enum PacketType {
  Greet = 1,
  Client = 2,
}

impl From<u8> for PacketType {
  fn from(num: u8) -> Self {
    match num {
      1 => Self::Greet,
      2 => Self::Client,
      _ => panic!("")
    }
  }
}