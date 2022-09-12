
// ===== Imports =====
use bytes::{BufMut, BytesMut};
// ===================

#[derive(Clone)]
pub struct Packet {
  payload: BytesMut,
}

impl Packet {
  pub fn to_bytes(self) -> BytesMut {
    let mut byts = BytesMut::new();
    byts.put(self.payload);
    byts
  }

  pub fn from_bytes(byts: BytesMut) -> Self {
    let payload = BytesMut::from(&byts[..]);
    Self { payload }
  }
}
