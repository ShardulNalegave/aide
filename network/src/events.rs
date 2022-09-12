
// ===== Imports =====
use message_io::network::Endpoint;
use crate::packet::Packet;
// ===================

pub enum Event {
  Connected(Endpoint),
  Disconnected(Endpoint),
  PacketReceived(Endpoint, Packet),
}