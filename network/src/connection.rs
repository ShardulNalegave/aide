
// ===== Imports =====
use std::collections::HashMap;
use message_io::network::Endpoint;
// ===================

pub type Connections = HashMap<Endpoint, Connection>;

pub struct Connection {
  endpoint: Endpoint,
  name: String,
}

impl Connection {
  pub(crate) fn new(name: String, endpoint: Endpoint) -> Self {
    Self { name, endpoint }
  }
}