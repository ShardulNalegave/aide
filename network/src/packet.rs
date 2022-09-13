
// ===== Imports =====
use uuid::{Bytes, Uuid};
use bytes::{Buf, BufMut, BytesMut};
// ===================

#[derive(Clone)]
pub struct Packet {
  pub header: Header,
  pub payload: BytesMut,
}

impl Packet {
  pub fn to_bytes(self) -> BytesMut {
    let mut byts = BytesMut::new();
    byts.put(self.payload);
    byts
  }

  pub fn from_bytes(byts: BytesMut) -> Self {
    let header_byts = BytesMut::from(&byts[..=32]);
    let header = Header::from_bytes(header_byts);
    let payload = BytesMut::from(&byts[33..]);
    Self { header, payload }
  }
}

pub struct Header {
  from: Uuid,
  to: Uuid,
}

impl Header {
  pub fn to_bytes(self) -> BytesMut {
    let mut byts = BytesMut::new();
    byts.put_u128(self.from.as_u128());
    byts.put_u128(self.to.as_u128());
    byts
  }

  pub fn from_bytes(mut byts: BytesMut) -> Self {
    let from = Uuid::from_u128(byts.get_u128());
    let to = Uuid::from_u128(byts.get_u128());
    Self { from, to }
  }
}