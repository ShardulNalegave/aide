
// ===== Imports =====
use network::prelude::*;
use std::io::Error;
// ===================

#[tokio::main]
async fn main() -> Result<(), Error> {
  let node = Node::new("Test Node", "127.0.0.1:5000")?;
  println!("Listening on 127.0.0.1:5000");
  let _task = node.run(|event| match event {
    Event::Connected => println!("New Connection"),
    Event::Disconnected => println!("Disconnected"),
  });

  Ok(())
}