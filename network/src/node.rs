use std::io::Error;
use message_io::network::Transport;
use message_io::node;
use message_io::node::{NodeHandler, NodeListener};

pub struct Node {
  name: String,
  host_addr: String,
  handler: NodeHandler<()>,
  listener: NodeListener<()>,
}

impl Node {
  pub fn new(name: &str, host_addr: &str) -> Result<Self, Error> {
    let (handler, listener) = node::split();
    handler.network().listen(Transport::FramedTcp, host_addr)?;

    Ok(Self {
      name: name.to_string(),
      host_addr: host_addr.to_string(),
      handler,
      listener,
    })
  }
}