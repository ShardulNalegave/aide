
// ===== Imports =====
use std::io::Error;
use std::sync::{Arc, Mutex};
use message_io::network::{NetEvent, Transport};
use message_io::node;
use message_io::node::{NodeEvent, NodeHandler, NodeListener, NodeTask};
use crate::connection::{Connection, Connections};
use crate::events::Event;
// ===================

pub fn new_node(name: &str, host_addr: &str, connections: Arc<Mutex<Connections>>) -> Result<(Node, NodeRunner), Error> {
  let (handler, listener) = node::split();
  let (_, _) = handler.network().listen(Transport::FramedTcp, host_addr)?;

  Ok((
    Node::new(name.to_string(), host_addr.to_string(), handler, connections.clone()),
    NodeRunner::new(name.to_string(), host_addr.to_string(), listener, connections),
  ))
}

pub struct Node {
  pub name: String,
  pub host_addr: String,
  handler: NodeHandler<()>,
  connections: Arc<Mutex<Connections>>,
}

impl Node {
  pub(crate) fn new(
    name: String,
    host_addr: String,
    handler: NodeHandler<()>,
    connections: Arc<Mutex<Connections>>,
  ) -> Self {
    Self { name, host_addr, handler, connections }
  }
}

pub struct NodeRunner {
  pub name: String,
  pub host_addr: String,
  listener: NodeListener<()>,
  connections: Arc<Mutex<Connections>>,
}

impl NodeRunner {
  pub(crate) fn new(
    name: String,
    host_addr: String,
    listener: NodeListener<()>,
    connections: Arc<Mutex<Connections>>,
  ) -> Self {
    Self { name, host_addr, listener, connections }
  }

  pub fn run(self, mut event_handler: impl FnMut(Event) + Send + 'static) -> NodeTask {
    let mut connections = self.connections;
    let listener = self.listener;
    listener.for_each_async(move |event| match event {
      NodeEvent::Network(net_event) => match net_event {
        NetEvent::Connected(endpoint, _ready) => {
          let conn = Connection::new();
          connections.lock().expect("Couldn't access connections hashmap.")
            .insert(endpoint, conn);
          event_handler(Event::Connected(endpoint))
        },
        NetEvent::Accepted(_, _) => {}
        NetEvent::Message(_, _) => {}
        NetEvent::Disconnected(endpoint) => event_handler(Event::Disconnected(endpoint)),
      }
      NodeEvent::Signal(_) => {}
    })
  }
}