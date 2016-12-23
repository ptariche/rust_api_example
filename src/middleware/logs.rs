use nickel::{Request, Response, MiddlewareResult};

pub fn enable_logs<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  println!("Method: {:?} | Path: {:?} | Source: {:?}", req.origin.method, req.origin.uri, req.origin.remote_addr);
  res.next_middleware()
}