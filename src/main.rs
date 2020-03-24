#![warn(rust_2018_idioms)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn index(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("It works!")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let addr = ([127, 0, 0, 1], 4242).into();

  let server = Server::bind(&addr).serve(make_service_fn(|_| async {
    Ok::<_, hyper::Error>(service_fn(index))
  }));

  println!("Listening on http://{}.", addr);

  Ok(server.await?)
}
