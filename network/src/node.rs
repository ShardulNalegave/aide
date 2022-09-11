use std::collections::HashMap;
use std::io::Error;
use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node;
use message_io::node::{NodeEvent, NodeHandler, NodeListener, NodeTask};
use crate::connection::Connection;
use crate::events::{Event};

#[allow(dead_code)]
pub struct Node {
  pub name: String,
  pub host_addr: String,
  handler: NodeHandler<()>,
  listener: NodeListener<()>,
  pub connections: HashMap<Endpoint, Connection>,
}

impl Node {
  pub fn new(name: &str, host_addr: &str) -> Result<Self, Error> {
    let (handler, listener) = node::split();
    handler.network().listen(Transport::FramedTcp, host_addr)?;

    Ok(Self {
      handler, listener,
      name: name.to_string(),
      host_addr: host_addr.to_string(),
      connections: HashMap::new(),
    })
  }

  pub fn run(self, mut event_handler: impl FnMut(Event) + Send + 'static) -> NodeTask {
    let listener = self.listener;
    listener.for_each_async(move |event| match event {
      NodeEvent::Network(net_event) => match net_event {
        NetEvent::Connected(_, _) => event_handler(Event::Connected),
        NetEvent::Accepted(_, _) => {}
        NetEvent::Message(_, _) => {}
        NetEvent::Disconnected(_) => event_handler(Event::Disconnected),
      }
      NodeEvent::Signal(_) => {}
    })
  }
}
