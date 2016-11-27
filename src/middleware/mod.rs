/// Middleware

use std::sync::{Arc, Mutex};

use super::iron::{Request, IronResult, IronError, Handler, Response, status};
use super::iron::middleware::{BeforeMiddleware, AroundMiddleware};
use super::iron::headers::Authorization;
use super::iron::modifiers::RedirectRaw;

use super::sessions::SessionStore;

type SStore = Arc<Mutex<SessionStore>>;


/// Simple info logger to display the incoming request method & url
pub struct InfoLog;
impl InfoLog {
    pub fn new() -> InfoLog {
        InfoLog{}
    }
}
impl BeforeMiddleware for InfoLog {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        println!("[{:?}]: {}", request.method, request.url);
        Ok(())
    }
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        Err(err)
    }
}


/// Session middleware handler to look for and insert the current
/// session into the request.extensions typemap.
/// This handler is intended to be returned from
/// AroundMiddleware->SessionMiddleware
struct SessionMiddlewareHandler<H: Handler> {
    store: SStore,
    handler: H,
}
impl<H: Handler> Handler for SessionMiddlewareHandler<H> {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        { // move to inner scope so store lock gets dropped before calling the given handle
            let mut store = self.store.lock().unwrap();
            let sess = match request.headers.get::<Authorization<String>>() {
                Some(token) => match store.get_mut(&token) {
                    Some(session) => Some(session),
                    _ => None,
                },
                _ => None,
            };
            println!("in around!");
            let curr_path = request.url.path().iter().map(|p| p.to_string()).next().unwrap_or("".to_string());
            if sess.is_none() && curr_path != "login" {
                return Ok(Response::with((status::Unauthorized, "please login")))
            }
        }
        self.handler.handle(request)
    }
}

/// SessionMiddleware (AroundMiddleware) intended to check incoming
/// requests for a session-token and reject any non token requests
pub struct SessionMiddleware {
    store: SStore,
}
impl SessionMiddleware {
    pub fn new(store: SStore) -> SessionMiddleware {
        SessionMiddleware {
            store: store,
        }
    }
}
impl AroundMiddleware for SessionMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(SessionMiddlewareHandler {
            store: self.store,
            handler: handler,
        }) as Box<Handler>
    }
}
