use log::{error, info};
use pin_utils::{unsafe_unpinned, unsafe_pinned};
use std::{collections::VecDeque, pin::Pin, task::{Context, Poll}};

use futures::Sink;
use tokio::{io::AsyncWrite, net::tcp::OwnedWriteHalf};

#[derive(Debug)]
#[must_use = "Streams do nothong, unless polled"]
pub struct CncSender {
  tx: OwnedWriteHalf,
  buffer: VecDeque<String>, 
}

impl CncSender {
  unsafe_pinned!(tx: OwnedWriteHalf);
  unsafe_unpinned!(buffer: VecDeque<String>);

  pub fn new(tx: OwnedWriteHalf) -> Self {
    CncSender { tx, buffer: VecDeque::new() }
  }

  fn send_all(self: &mut Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>>{
    while let Some(i) = self.as_mut().buffer().pop_front() {
      match self.as_mut().tx().poll_write(cx, i.as_bytes()) {
        Poll::Ready(Ok(size)) => {
          info!("ok poll_write size {}", size);
          if size == i.as_bytes().len() {
            continue;
          }else {
            log::error!("Seems, like not all bytes written");
          }
        }
        Poll::Ready(Err(err)) => {
          error!("Error {}", err);
          return Poll::Ready(Err(err));
        }

        Poll::Pending => {
          return Poll::Pending;
        }
      }
    };
    Poll::Ready(Ok(()))
  }
}

// struct CncSenderError(String);

impl Sink<String> for CncSender {
  type Error = std::io::Error;

  fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    println!("poll_ready");
    Poll::Ready(Ok(()))
  }

  fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    println!("poll_flush");
    Self::send_all(&mut self, cx)
  }

  fn start_send(mut self: Pin<&mut Self>, item: String) -> Result<(), Self::Error> {
    println!("start_send");
    self.as_mut().buffer().push_back(item);
    Ok(())
  }

  fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    println!("poll_close");
    match Self::send_all(&mut self, cx) {
      Poll::Pending => Poll::Pending,
      Poll::Ready(Err(e)) => {
        Poll::Ready(Err(e))
      }
      Poll::Ready(Ok(())) => {
        self.as_mut().tx().poll_shutdown(cx)
      }
    }
  }
}
