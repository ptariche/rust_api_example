use std::io::Write;
use nickel::{NickelError, Request, Action};
use nickel::status::StatusCode;

pub fn enable_handler<D>(err: &mut NickelError<D>, req: &mut Request<D>) -> Action {

  println!("[{}] ERROR: {}",
    req.path_without_query().unwrap(),
    err.message);

  if let Some(ref mut res) = err.stream {
    match res.status() {
      StatusCode::BadRequest => {
        let _ = res.write_all(err.message.as_bytes());
        return Action::Halt(())
      }
      StatusCode::NotFound => {
        let _ = res.write_all(err.message.as_bytes());
        return Action::Halt(())
      }
      _ => {}
    }
  }

  Action::Continue(())
}
