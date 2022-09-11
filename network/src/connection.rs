
// ===== Imports =====
use std::collections::HashMap;
use message_io::network::Endpoint;
// ===================

pub type Connections = HashMap<Endpoint, Connection>;

pub struct Connection {
  //
}

impl Connection {
  pub(crate) fn new() -> Self {
    Self {}
  }
}