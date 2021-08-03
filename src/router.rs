use crate::endpoints;
use crate::options::InputOptions;
use hyper::Body;
use routerify::Router;

pub fn router(
  input_options: InputOptions,
) -> Router<Body, Box<dyn std::error::Error + Send + Sync>> {

  Router::builder()
    .data(input_options)
    .any_method("/ws", endpoints::upgrade_ws(endpoints::on_ws_connect))
    .get("/config.json", endpoints::config)
    .post("/ngc", endpoints::post_ngc_file)
    .any(endpoints::serve_static)
    .build()
    .unwrap()

}
