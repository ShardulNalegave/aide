use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_stream::{Stream, StreamExt};

pub struct EventStream {
  to_send: Vec<Event>,
}

impl EventStream {
  pub fn new() -> Self {
    EventStream { to_send: vec![] }
  }

  pub fn register_event(&mut self, event: Event) {
    self.to_send.push(event);
  }
}

impl Stream for EventStream {
  type Item = Event;

  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    if self.to_send.is_empty() {
      Poll::Pending
    } else {
      let val = self.to_send.remove(0);
      Poll::Ready(Some(val))
    }
  }
}

impl StreamExt for EventStream {}

pub enum Event {
  Connected,
  Disconnected,
}