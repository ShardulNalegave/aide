use std::collections::HashMap;
use std::io::Error;
use message_io::network::{Endpoint, Transport};
use message_io::node;
use message_io::node::{NodeEvent, NodeHandler, NodeListener};
use crate::connection::Connection;

pub struct Node {
  name: String,
  host_addr: String,
  handler: NodeHandler<()>,
  listener: NodeListener<()>,
  pub connections: HashMap<Endpoint, Connection>,
}

impl Node {
  pub fn new(name: &str, host_addr: &str) -> Result<Self, Error> {
    let (handler, listener) = node::split();
    handler.network().listen(Transport::FramedTcp, host_addr)?;

    let stream = tokio_stream::iter(&[]);

    Ok(Self {
      handler, listener,
      name: name.to_string(),
      host_addr: host_addr.to_string(),
      connections: HashMap::new(),
    })
  }

  pub fn run(mut self) {
    let listener = self.listener;
    listener.for_each(move |event| match event {
      NodeEvent::Network(_) => {}
      NodeEvent::Signal(_) => {}
    });
  }
}
