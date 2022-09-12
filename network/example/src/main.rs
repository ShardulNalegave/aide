
// ===== Imports =====
use network::prelude::*;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
// ===================

#[tokio::main]
async fn main() -> Result<(), Error> {
  let args: Vec<String> = std::env::args().collect();
  let run_as = args[1].clone();
  let host_addr = args[2].clone();
  let connect_to_addr = args[3].clone();

  let connections: Connections = HashMap::new();
  let (mut node, runner) = new_node("Test Node", &host_addr, Arc::new(Mutex::new(connections)))?;

  let _task = runner.run(move |event| match event {
    Event::Connected(endpoint) => println!("Connected: {:?}", endpoint),
    Event::Disconnected(endpoint) => println!("Disconnected: {:?}", endpoint),
  });

  if run_as == "client" {
    let _endpoint = node.connect(&connect_to_addr)?;
  }

  Ok(())
}