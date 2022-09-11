
// ===== Imports =====
use network::prelude::*;
use std::io::Error;
// ===================

#[tokio::main]
async fn main() -> Result<(), Error> {
  let args: Vec<String> = std::env::args().collect();
  let host_addr = args[1].clone();
  
  let node = Node::new("Test Node", &host_addr)?;
  println!("Listening on ${:?}", host_addr);
  let _task = node.run(|event| match event {
    Event::Connected => println!("New Connection"),
    Event::Disconnected => println!("Disconnected"),
  });

  Ok(())
}