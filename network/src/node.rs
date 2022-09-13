
// ===== Imports =====
use std::io::Error;
use std::sync::{Arc, Mutex};
use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node;
use message_io::node::{NodeEvent, NodeHandler, NodeListener, NodeTask};
use uuid::Uuid;
use crate::connection::{Connection, Connections};
use crate::events::Event;
use crate::packet::Packet;
// ===================

pub fn new_node(name: Uuid, host_addr: &str, connections: Arc<Mutex<Connections>>) -> Result<(Node, NodeRunner), Error> {
  let (handler, listener) = node::split();
  let (_, _) = handler.network().listen(Transport::FramedTcp, host_addr)?;

  Ok((
    Node::new(name, host_addr.to_string(), handler, connections.clone()),
    NodeRunner::new(name, host_addr.to_string(), listener, connections),
  ))
}

// ============================================================================================

pub struct Node {
  pub name: Uuid,
  pub host_addr: String,
  handler: NodeHandler<()>,
  connections: Arc<Mutex<Connections>>,
}

impl Node {
  pub(crate) fn new(
    name: Uuid,
    host_addr: String,
    handler: NodeHandler<()>,
    connections: Arc<Mutex<Connections>>,
  ) -> Self {
    Self { name, host_addr, handler, connections }
  }

  pub fn connect(&mut self, addr: &str) -> Result<Endpoint, Error> {
    let (endpoint, _) = self.handler.network().connect(Transport::FramedTcp, addr.to_string())?;
    Ok(endpoint)
  }

  pub fn send_packet(&mut self, endpoint: Endpoint, packet: Packet) {
    let byts = packet.to_bytes();
    self.handler.network().send(endpoint, &byts);
  }
}

// ============================================================================================

pub struct NodeRunner {
  pub name: Uuid,
  pub host_addr: String,
  listener: NodeListener<()>,
  connections: Arc<Mutex<Connections>>,
}

impl NodeRunner {
  pub(crate) fn new(
    name: Uuid,
    host_addr: String,
    listener: NodeListener<()>,
    connections: Arc<Mutex<Connections>>,
  ) -> Self {
    Self { name, host_addr, listener, connections }
  }

  pub fn run(self, mut event_handler: impl FnMut(Event) + Send + 'static) -> NodeTask {
    let connections = self.connections;
    let listener = self.listener;
    listener.for_each_async(move |event| match event {
      NodeEvent::Network(net_event) => match net_event {
        NetEvent::Connected(endpoint, _ready) => {
          let conn = Connection::new("".to_string(), endpoint);
          connections.lock().expect("Couldn't access connections hashmap.")
            .insert(endpoint, conn);
          event_handler(Event::Connected(endpoint))
        },
        NetEvent::Accepted(endpoint, _) => {
          let conn = Connection::new("".to_string(), endpoint);
          connections.lock().expect("Couldn't access connections hashmap.")
            .insert(endpoint, conn);
          event_handler(Event::Connected(endpoint))
        }
        NetEvent::Message(endpoint, byts) => {
          let packet = Packet::from_bytes(byts.into());
          event_handler(Event::PacketReceived(endpoint, packet))
        }
        NetEvent::Disconnected(endpoint) => event_handler(Event::Disconnected(endpoint)),
      }
      NodeEvent::Signal(_) => {}
    })
  }
}