use pin_utils::{unsafe_pinned, unsafe_unpinned};
use std::{pin::Pin, process::Output, task::{Context, Poll}};

use futures::{Future, FutureExt, Stream};
use tokio::{io::{AsyncRead, ReadBuf}, net::tcp::OwnedReadHalf};
use log::info;

#[derive(Debug)]
#[must_use = "Streams do nothong, unless polled"]
pub struct CncReceiver {
  commands: Vec<String>,
  rx: OwnedReadHalf,
}


impl CncReceiver {
  unsafe_pinned!(rx: OwnedReadHalf);
  unsafe_unpinned!(commands: Vec<String>);

  pub fn new(rx: OwnedReadHalf) -> Self {
    CncReceiver { rx, commands: Vec::new()}
  }
}

impl Stream for CncReceiver {
  type Item = String;
  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let mut buffer = [0; 1024];
    let mut buffer = ReadBuf::new(&mut buffer);
    match self.as_mut().rx().poll_read(cx, &mut buffer) {
      
      Poll::Ready(Err(e)) => {
        log::error!("error in rx: {}", e);
        return Poll::Ready(None);
      }
      Poll::Ready(Ok(_)) => {
        info!("Got some bytes");
        let mut data = Vec::new();
        data.extend_from_slice(buffer.filled());
        let data = String::from_utf8(data).unwrap();
        self.as_mut().commands().extend(
          data
            .split('\n')
            .map(From::from)
            .collect::<Vec<String>>(),
        );
      }
      _ => {  }
    };

    
    if self.as_mut().commands().is_empty() {
      Poll::Pending
    } else {
      let first = self.as_mut().commands().remove(0);
      Poll::Ready(Some(first))
    }
  }
}
