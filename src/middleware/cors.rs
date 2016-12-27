use nickel::{Request, Response, MiddlewareResult};
use hyper::header::{AccessControlAllowOrigin, AccessControlAllowHeaders};

pub fn enable_cors<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
  res.set(AccessControlAllowOrigin::Any);
  res.set(AccessControlAllowHeaders(vec![
    "Origin".into(),
    "X-Requested-With".into(),
    "Content-Type".into(),
    "Accept".into(),
  ]));

  res.next_middleware()
}
