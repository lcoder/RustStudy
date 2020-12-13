pub trait Service {
  type Request;
  type Response;
  type Error;
  type Future;
  fn call(&self, req: Self::Request) -> Self::Future;
}

trait HttpService = Service<
  Request = http::Request,
  Response = http::Response,
  Error = http::Error
>;


fn main() {
  
}

