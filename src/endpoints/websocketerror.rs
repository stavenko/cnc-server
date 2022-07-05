use derive_more::Display;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Display)]
#[display(fmt = "routerify-websocket: {}")]
pub enum WebsocketError {
  /// Websocket upgrade error.
  #[display(fmt = "Websocket upgrade error: {}", _0)]
  Upgrade(BoxError),

  /// Failed to decode the message data as `JSON` in [`message.decode_json()`](./struct.Message.html#method.decode_json) method.
  #[cfg(feature = "json")]
  #[display(fmt = "Failed to decode the message data as JSON: {}", _0)]
  DecodeJson(BoxError),

  /// Failed to convert a struct to `JSON` in [`message.json()`](./struct.Message.html#method.json) method.
  #[cfg(feature = "json")]
  #[display(fmt = "Failed to convert a struct to JSON: {}", _0)]
  EncodeJson(BoxError),
}
