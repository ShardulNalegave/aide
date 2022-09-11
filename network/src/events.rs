
// ===== Imports =====
use message_io::network::Endpoint;
// ===================

pub enum Event {
  Connected(Endpoint),
  Disconnected(Endpoint),
}